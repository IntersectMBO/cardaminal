use miette::IntoDiagnostic;
use tracing::info;

use crate::{
    transaction::model::staging::StagingTransaction,
    wallet::{config::Wallet, dal::WalletDB},
    Context,
};

pub async fn with_staging_tx<F>(wallet: &str, id: i32, ctx: &Context, op: F) -> miette::Result<()>
where
    F: FnOnce(StagingTransaction) -> miette::Result<StagingTransaction>,
{
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let mut record = wallet_db
        .fetch_by_id(&id)
        .await
        .into_diagnostic()?
        .ok_or(miette::miette!("transaction doesn't exist"))?;

    let mut tx: StagingTransaction = serde_json::from_slice(&record.tx_json).into_diagnostic()?;

    tx = op(tx)?;

    record.tx_json = serde_json::to_vec(&tx).into_diagnostic()?;

    wallet_db
        .update_transaction(record)
        .await
        .into_diagnostic()?;

    info!("transaction updated");

    Ok(())
}
