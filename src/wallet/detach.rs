use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Wallet name to detach
    wallet: String,
}

#[instrument("detach", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("Wallet {} detached", args.wallet);
    Ok(())
}
