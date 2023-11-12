use miette::IntoDiagnostic;
use tracing::info;

use crate::{
    transaction::model::staging::StagingTransaction,
    wallet::{config::Wallet, dal::WalletDB},
};

pub async fn with_staging_tx<F>(ctx: &super::EditContext<'_>, op: F) -> miette::Result<()>
where
    F: FnOnce(StagingTransaction) -> StagingTransaction,
{
    let wallet = Wallet::load_config(&ctx.global_ctx.dirs.root_dir, &ctx.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(
        &wallet.name,
        &Wallet::dir(&ctx.global_ctx.dirs.root_dir, &wallet.name),
    )
    .await
    .into_diagnostic()?;

    let mut record = wallet_db
        .fetch_by_id(&(ctx.tx_id as i32))
        .await
        .into_diagnostic()?
        .ok_or(miette::miette!("transaction doesn't exist"))?;

    let mut tx: StagingTransaction = serde_json::from_slice(&record.tx_json).into_diagnostic()?;

    tx = op(tx);

    record.tx_json = serde_json::to_vec(&tx).into_diagnostic()?;

    wallet_db
        .update_transaction(record)
        .await
        .into_diagnostic()?;

    info!("transaction updated");

    Ok(())
}
