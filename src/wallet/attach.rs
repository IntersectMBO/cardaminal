use clap::Parser;
use miette::bail;
use tracing::{info, instrument};

use crate::{chain::config::Chain, wallet::config::Wallet};

#[derive(Parser)]
pub struct Args {
    /// Wallet name to attach
    wallet: String,
    /// Chain name to attach
    chain: String,
}

#[instrument("attach", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?;
    if wallet.is_none() {
        bail!("wallet doesn't exist")
    }

    if !Chain::dir(&ctx.dirs.root_dir, &args.chain).exists() {
        bail!("chain doesn't exist")
    }

    let mut wallet = wallet.unwrap();
    // TODO: to enable the option to replace the chain, it's necessary to validate if already exists data from the old chain
    if wallet.chain.is_some() {
        bail!("wallet already attached")
    }

    wallet.chain = Some(args.chain.clone());

    wallet.save_config(&ctx.dirs.root_dir)?;

    info!(wallet = args.wallet, chain = args.chain, "attached",);
    Ok(())
}
