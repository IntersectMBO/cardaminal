use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Wallet name to attach
    wallet: String,
    /// Chain name to attach
    chain: String,
}

#[instrument("attach", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("Wallet {} attached to {} chain", args.wallet, args.chain);
    Ok(())
}
