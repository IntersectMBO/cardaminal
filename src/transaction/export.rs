use std::{fs, io::Write, path::PathBuf};

use clap::Parser;
use miette::IntoDiagnostic;
use tracing::instrument;

use crate::{
    transaction::model::staging::StagingTransaction,
    wallet::{config::Wallet, dal::WalletDB},
};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,

    /// transaction id
    id: i32,

    /// output to save json file
    #[arg(long, action)]
    output_path: Option<PathBuf>,
}

#[instrument("export", skip_all, fields(wallet=args.wallet,id=args.id))]
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

    let staging_transaction: StagingTransaction =
        serde_json::from_slice(&transaction.tx_json).into_diagnostic()?;

    let json = serde_json::to_vec_pretty(&staging_transaction).into_diagnostic()?;

    let output_path = args.output_path.unwrap_or(PathBuf::new());

    let mut file = fs::File::create(output_path.join(format!("tx{}.json", transaction.id)))
        .into_diagnostic()?;
    file.write_all(&json).into_diagnostic()?;

    Ok(())
}
