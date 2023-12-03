use clap::Parser;
use miette::{bail, IntoDiagnostic};
use tracing::{info, instrument};

use pallas::txbuilder::{BuildBabbage, StagingTransaction};

use crate::wallet::{
    config::Wallet,
    dal::{entities::transaction::Status, WalletDB},
};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,

    /// transaction id
    id: i32,
}

#[instrument("build", skip_all, fields())]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let mut record = wallet_db
        .fetch_by_id(&args.id)
        .await
        .into_diagnostic()?
        .ok_or(miette::miette!("transaction doesn't exist"))?;

    if record.status != Status::Staging {
        bail!("can only build transactions in staging state")
    }

    let tx: StagingTransaction = serde_json::from_slice(&record.tx_json).into_diagnostic()?;

    // TODO: `build` will take protocol parameters from wallet db for validated tx building

    let built_tx = tx
        .build_babbage_raw()
        .map_err(|e| miette::miette!("tx build failed: {e:?}"))?;

    record.status = Status::Built;
    record.tx_json = serde_json::to_vec(&built_tx).into_diagnostic()?;
    record.tx_cbor = Some(built_tx.tx_bytes.0);

    wallet_db
        .update_transaction(record)
        .await
        .into_diagnostic()?;

    info!("transaction built");

    Ok(())
}
