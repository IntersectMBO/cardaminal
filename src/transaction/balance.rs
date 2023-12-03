use clap::Parser;
use miette::{bail, Context, IntoDiagnostic};
use pallas::ledger::traverse::Era;
use tracing::instrument;
use pallas::txbuilder::StagingTransaction;

use crate::wallet::{config::Wallet, dal::{WalletDB, entities::transaction::Status}};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,

    /// transaction id
    id: i32,

    /// output the absolute value (no sign)
    #[arg(short, long, action)]
    absolute: bool,
}

#[instrument("export", skip_all, fields(wallet=args.wallet,id=args.id))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let record = wallet_db
        .fetch_by_id(&args.id)
        .await
        .into_diagnostic()?
        .ok_or(miette::miette!("transaction doesn't exist"))?;

    if record.status != Status::Staging {
        bail!("balance can only be called during building")
    }

    let tx: StagingTransaction = serde_json::from_slice(&record.tx_json).into_diagnostic()?;

    let total_inputs = compute_total_input(&tx, &wallet_db).await?;
    let total_outputs = compute_total_output(&tx);
    let fee = tx.fee.unwrap_or_default();

    let mut result = (total_inputs as i128) - (total_outputs as i128) - (fee as i128);

    if args.absolute {
        result = result.abs();
    }

    println!("{result}");

    Ok(())
}

async fn compute_total_input(tx: &StagingTransaction, wallet: &WalletDB) -> miette::Result<u64> {
    let mut total = 0;

    if let Some(inputs) = &tx.inputs {
        for input in inputs.iter() {
            let resolved = wallet
                .resolve_utxo(&input.tx_hash.0, input.txo_index as i32)
                .await
                .into_diagnostic()
                .context("resolving input")?;

            let resolved = match resolved {
                Some(x) => x,
                None => bail!("can't find required utxo in wallet"),
            };

            let era = Era::try_from(resolved.era)
                .into_diagnostic()
                .context("parsing utxo era")?;

            let parsed = pallas::ledger::traverse::MultiEraOutput::decode(era, &resolved.cbor)
                .into_diagnostic()
                .context("parsing utxo cbor")?;

            total += parsed.lovelace_amount();
        }
    }

    Ok(total)
}

fn compute_total_output(tx: &StagingTransaction) -> u64 {
    let mut total = 0;

    if let Some(outputs) = &tx.outputs {
        for output in outputs.iter() {
            total += output.lovelace;
        }
    }

    total
}
