use clap::Parser;
use miette::IntoDiagnostic;
use tracing::{info, instrument};

use crate::{
    transaction::model::staging::{Input, StagingTransaction},
    wallet::{config::Wallet, dal::WalletDB},
};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,
    /// transaction id
    id: i32,
    /// utxo hash
    utxo_hash: String,
    /// utxo idx
    utxo_idx: usize,
}

#[instrument("add", skip_all, fields())]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let mut transaction = wallet_db
        .fetch_by_id(&args.id)
        .await
        .into_diagnostic()?
        .ok_or(miette::miette!("transaction doesn't exist"))?;

    let mut staging_transaction: StagingTransaction =
        serde_json::from_slice(&transaction.tx_json).into_diagnostic()?;

    let mut inputs = staging_transaction.inputs.unwrap_or(vec![]);

    let input = Input::new(args.utxo_hash.try_into()?, args.utxo_idx);
    inputs.push(input);

    staging_transaction.inputs = Some(inputs);

    transaction.tx_json = serde_json::to_vec(&staging_transaction).into_diagnostic()?;

    wallet_db
        .update_transaction(transaction)
        .await
        .into_diagnostic()?;

    info!("transaction updated");

    Ok(())
}
