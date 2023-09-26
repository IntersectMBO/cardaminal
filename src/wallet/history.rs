use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Wallet name to get history
    wallet: String,
}

#[instrument("history", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    for i in 0..3 {
        info!(transaction = i, "update");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    Ok(())
}
