use std::{collections::HashMap, iter, thread, time::Duration};

use clap::Parser;
use indicatif::ProgressStyle;
use miette::{bail, IntoDiagnostic};
use pallas::{
    codec::minicbor,
    crypto::hash::Hash,
    ledger::{
        addresses::{Address, ShelleyPaymentPart},
        traverse::{Era, MultiEraBlock, MultiEraOutput, MultiEraUpdate},
    },
};
use tracing::{info, info_span, instrument, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::{chain::config::Chain, wallet::{dal::WalletDB, config::Wallet}};

#[derive(Parser)]
pub struct Args {
    /// Wallet name to history update
    wallet: String,
}

#[instrument("update", skip_all, fields(wallet=args.wallet))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    info!(chain = args.wallet, "updating");

    let chain_name = match Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)? {
        Some(cfg) => match cfg.chain {
            Some(n) => n,
            None => bail!("wallet not attached to a chain")
        },
        None => bail!("wallet doesn't exist"),
    };

    let chain_db = Chain::load_db(&ctx.dirs.root_dir, &chain_name)?;

    let chain_tip = match chain_db.find_tip().into_diagnostic()? {
        Some(tip) => tip,
        None => bail!("chain db empty"),
    };

    let wallet_path = ctx
        .dirs
        .root_dir
        .join("wallets")
        .join(slug::slugify(&args.wallet));

    if !wallet_path.exists() {
        bail!("could not find a wallet named '{}'", &args.wallet)
    }

    let wallet_db = WalletDB::open(&args.wallet, wallet_path)
        .await
        .into_diagnostic()?;

    // intersect wallet db with chain

    let mut recent_points = wallet_db.paginate_recent_points(None);

    let intersect = if recent_points.fetch().await.into_diagnostic()?.is_empty() {
        None
    } else {
        let mut found_intersect = None;

        'outer: while let Some(points) = recent_points.fetch_and_next().await.into_diagnostic()? {
            for point in points {
                let block_slot = point.slot as u64;
                let block_hash: [u8; 32] = point.block_hash.try_into().unwrap();

                if chain_db
                    .chain_contains(block_slot, &block_hash.into())
                    .into_diagnostic()?
                {
                    found_intersect = Some((block_slot, Hash::new(block_hash)));
                    break 'outer;
                }
            }
        }

        match found_intersect {
            Some(p) => {
                wallet_db
                    .rollback_to_slot(p.0)
                    .await
                    .into_diagnostic()?;

                Some(p)
            },
            None => bail!("could not intersect wallet db with chain"),
        }
    };

    // start updating

    let chain_iter = chain_db
        .read_chain_range(intersect, chain_tip)
        .into_diagnostic()?;

    // set up pb

    let span = info_span!("wallet-update");
    span.pb_set_style(&ProgressStyle::default_bar());
    span.pb_set_length(chain_tip.0);

    span.pb_set_style(
        &ProgressStyle::with_template(
            "{spinner:.white} [{elapsed_precise}] [{wide_bar:.white/white}] {pos}/{len}",
        )
        .unwrap(),
    );

    let span_enter = span.enter();

    // crawl

    if let Some(mut iter) = chain_iter {
        // crawl range is inclusive, skip first point as already processed
        iter.next();

        for point in iter {
            let point = point.into_diagnostic()?;

            let block_bytes = match chain_db.get_block(point.1).into_diagnostic()? {
                Some(b) => b,
                None => bail!("could not find block in chain db"),
            };

            let block = MultiEraBlock::decode(&block_bytes).into_diagnostic()?;

            // dummy
            let wallet_pkhs = vec![[0u8; 28], [1; 28]];

            process_block(&wallet_db, &block, wallet_pkhs).await?;

            info!(last_slot = point.0, "new blocks crawled");
            Span::current().pb_set_position(point.0);
        }
    }

    std::mem::drop(span_enter);
    std::mem::drop(span);

    info!("wallet updated");

    Ok(())
}

/// Given a block and a list of public key hashes controlled by the wallet,
/// modify the different Wallet DB tables according to the contents of the
/// block in relation to the wallet.
pub async fn process_block(
    wallet_db: &WalletDB,
    block: &MultiEraBlock<'_>,
    wallet_pkhs: Vec<[u8; 28]>,
) -> miette::Result<()> {
    let txs = block.txs().clone();

    // UTxOs

    let produced_for_wallet = txs
        .iter()
        .map(|tx| iter::repeat(*tx.hash()).zip(tx.produces()))
        .flatten()
        .map(|(txid, (idx, txo))| (txid, idx, txo, block.slot()))
        .filter(|(_, _, txo, _)| output_controlled_by_pkh(txo, &wallet_pkhs))
        .collect::<Vec<_>>();

    let consumed = txs
        .iter()
        .map(|tx| tx.consumes())
        .flatten()
        .collect::<Vec<_>>();

    wallet_db
        .insert_utxos(produced_for_wallet)
        .await
        .into_diagnostic()?;

    let removed = wallet_db.remove_utxos(consumed).await.into_diagnostic()?;

    // Transaction History

    let input_resolver = removed
        .into_iter()
        .map(|u| ((u.tx_hash, u.txo_index as u64), (u.era, u.cbor)))
        .collect::<HashMap<_, _>>();

    for (blk_idx, tx) in txs.iter().enumerate() {
        let mut involved = false;
        let mut value_deltas: HashMap<Vec<u8>, HashMap<Vec<u8>, i128>> = HashMap::new();

        // process inputs
        for input in tx.consumes() {
            if let Some((era, txo_cbor)) =
                input_resolver.get(&(input.hash().to_vec(), input.index()))
            {
                involved = true;

                let era = match era {
                    0 => Era::Byron,
                    1 => Era::Alonzo,
                    2 => Era::Babbage,
                    _ => bail!("unrecognised era in db"),
                };

                let txo = MultiEraOutput::decode(era, txo_cbor).into_diagnostic()?;

                *value_deltas
                    .entry(vec![])
                    .or_default()
                    .entry(vec![])
                    .or_default() -= txo.lovelace_amount() as i128;

                for asset in txo.non_ada_assets().iter().map(|p| p.assets()).flatten() {
                    *value_deltas
                        .entry(asset.policy().to_vec())
                        .or_default()
                        .entry(asset.name().into())
                        .or_default() -= asset.output_coin().unwrap() as i128;
                }
            }
        }

        // process outputs
        for (_, output) in tx.produces() {
            if output_controlled_by_pkh(&output, &wallet_pkhs) {
                involved = true;

                *value_deltas
                    .entry(vec![])
                    .or_default()
                    .entry(vec![])
                    .or_default() += output.lovelace_amount() as i128;

                for asset in output.non_ada_assets().iter().map(|p| p.assets()).flatten() {
                    *value_deltas
                        .entry(asset.policy().to_vec())
                        .or_default()
                        .entry(asset.name().into())
                        .or_default() += asset.output_coin().unwrap() as i128;
                }
            }
        }

        /*
            TODO: check if wallet pkh signed the tx

            for pkh in wallet_pkhs {
                if tx.vkey_witnesses().contains...

                }
            }
        */

        let lovelace_delta = value_deltas
            .remove(&vec![])
            .unwrap()
            .remove(&vec![])
            .unwrap();

        // add history entry
        if involved {
            // TODO: value delta stored in WalletDB is currently just lovelace
            // I was going to include value delta as CBOR encoding of a Value
            // object, but Value can only hold u64, whereas we need negative
            // amounts and amounts larger than 64 bit (as we potentially sum
            // u64 values), for now it is just lovelace change i128 big endian
            wallet_db
                .insert_history_tx(
                    *tx.hash(),
                    block.slot(),
                    blk_idx as u16,
                    lovelace_delta.to_be_bytes().to_vec(),
                )
                .await
                .into_diagnostic()?;
        }
    }

    // Protocol Parameters

    for (blk_idx, tx) in txs.iter().enumerate().filter(|(_, x)| x.is_valid()) {
        if let Some(x) = tx.update() {
            let cbor = match x {
                MultiEraUpdate::AlonzoCompatible(u) => minicbor::to_vec(u).unwrap(),
                MultiEraUpdate::Babbage(u) => minicbor::to_vec(u).unwrap(),
                _ => unreachable!(),
            };

            wallet_db
                .insert_protocol_parameters(block.slot(), blk_idx as u16, cbor)
                .await
                .into_diagnostic()?
        }
    }

    // Recent Points

    wallet_db
        .insert_recent_point(block.slot(), *block.hash())
        .await
        .into_diagnostic()?;

    Ok(())
}

fn output_controlled_by_pkh(txo: &MultiEraOutput<'_>, pkhs: &Vec<[u8; 28]>) -> bool {
    let controlling_pkh = match txo.address().unwrap() {
        Address::Shelley(a) => match a.payment() {
            ShelleyPaymentPart::Key(h) => h.clone(),
            _ => return false,
        },
        _ => return false,
    };

    pkhs.contains(&controlling_pkh)
}
