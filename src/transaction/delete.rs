use clap::Parser;
use miette::IntoDiagnostic;
use tracing::{info, instrument};

use crate::wallet::{config::Wallet, dal::WalletDB};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,

    /// transaction id
    id: i32,
}

#[instrument("delete", skip_all, fields(wallet=args.wallet, id=args.id))]
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

    let confirm = inquire::Confirm::new(&format!(
        "Do you confirm deleting the transacion: {}?",
        &transaction.id
    ))
    .prompt()
    .into_diagnostic()?;

    if confirm {
        wallet_db
            .remove_transaction(&transaction.id)
            .await
            .into_diagnostic()?;
        info!("transaction deleted");
    }

    Ok(())
}
