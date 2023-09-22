use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Wallet name to detach
    wallet: String,
    /// Chain name to detach
    chain: String,
}

#[instrument("detach", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("Wallet {} detached to {} chain", args.wallet, args.chain);
    Ok(())
}
