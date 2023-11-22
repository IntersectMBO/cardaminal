use clap::Parser;
use miette::IntoDiagnostic;
use pallas::txbuilder::{BuiltTransaction, StagingTransaction};
use tracing::instrument;

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

#[instrument("inspect", skip_all, fields(wallet=args.wallet,id=args.id))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let transaction = wallet_db
        .fetch_by_id(&args.id)
        .await
        .into_diagnostic()?
        .ok_or(miette::miette!("transaction doesn't exist"))?;

    let json = match transaction.status {
        Status::Staging => {
            let staging_transaction: StagingTransaction =
                serde_json::from_slice(&transaction.tx_json).into_diagnostic()?;

            serde_json::to_string_pretty(&staging_transaction).unwrap()
        }
        _ => {
            let built_transaction: BuiltTransaction =
                serde_json::from_slice(&transaction.tx_json).into_diagnostic()?;

            serde_json::to_string_pretty(&built_transaction).unwrap()
        }
    };

    println!("{json}");

    Ok(())
}
