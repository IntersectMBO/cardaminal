use clap::Parser;
use miette::IntoDiagnostic;
use tracing::instrument;

use super::model::staging::StagingTransaction;
use crate::wallet::{config::Wallet, dal::WalletDB};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,
}

#[instrument("create", skip_all, fields(wallet=args.wallet))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let tx = StagingTransaction::default();
    let tx_json = serde_json::to_vec(&tx).into_diagnostic()?;

    let id = wallet_db
        .insert_transaction(tx_json)
        .await
        .into_diagnostic()?;

    println!("{id}");

    Ok(())
}
