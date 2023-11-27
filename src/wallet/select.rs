use clap::Parser;
use miette::{Context, IntoDiagnostic};
use pallas::ledger::traverse::{Era, MultiEraOutput};
use sea_orm::Order;
use tracing::instrument;

use crate::wallet::{config::Wallet, dal::WalletDB};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet to query
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    name: Option<String>,

    /// Only select first matching utxo
    #[arg(long)]
    first: bool,

    /// Min lovelace that utxo must hold
    #[arg(long)]
    min_lovelace: Option<u64>,

    /// Max lovelace that utxo must hold
    #[arg(long)]
    max_lovelace: Option<u64>,

    /// Should not hold native assets
    #[arg(long, action)]
    no_native_assets: bool,
}

#[instrument("utxos", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet_name = args
        .name
        .as_ref()
        .ok_or(miette::miette!("wallet param is required"))?;

    let wallet = Wallet::load_config(&ctx.dirs.root_dir, wallet_name)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let utxos = wallet_db
        .fetch_all_utxos(Order::Asc)
        .await
        .into_diagnostic()?;

    for utxo in utxos {
        let era = Era::try_from(utxo.era)
            .into_diagnostic()
            .context("parsing utxo era")?;

        let parsed = pallas::ledger::traverse::MultiEraOutput::decode(era, &utxo.cbor)
            .into_diagnostic()
            .context("parsing utxo cbor")?;

        if utxo_matches(&parsed, &args) {
            println!("{}#{}", hex::encode(utxo.tx_hash), utxo.txo_index);

            if args.first {
                return Ok(());
            }
        }
    }

    Ok(())
}

fn utxo_matches(utxo: &MultiEraOutput, args: &Args) -> bool {
    if let Some(min) = &args.min_lovelace {
        if utxo.lovelace_amount() < *min {
            return false;
        }
    }

    if let Some(max) = &args.max_lovelace {
        if utxo.lovelace_amount() > *max {
            return false;
        }
    }

    if args.no_native_assets {
        if !utxo.non_ada_assets().is_empty() {
            return false;
        }
    }

    return true;
}
