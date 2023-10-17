use clap::Parser;
use miette::{bail, IntoDiagnostic};
use tracing::{info, instrument};

use crate::wallet::config::Wallet;

#[derive(Parser)]
pub struct Args {
    /// Wallet name to detach
    wallet: String,
}

#[instrument("detach", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?;
    if wallet.is_none() {
        bail!("wallet doesn't exist")
    }

    let mut wallet = wallet.unwrap();
    if wallet.chain.is_none() {
        bail!("wallet hasn't been attached yet")
    }

    let confirm = inquire::Confirm::new(&format!(
        "Do you confirm detaching chain {} from the wallet {}?",
        wallet.chain.unwrap(),
        &wallet.name
    ))
    .prompt()
    .into_diagnostic()?;

    if confirm {
        // TODO: check if it's need necessary to delete all data from the old chain from this wallet (sqlite)
        wallet.chain = None;
        wallet.save_config(&ctx.dirs.root_dir)?;

        info!(wallet = args.wallet, "detached");
    }

    Ok(())
}
