use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Wallet name to get history
    #[arg(short, long)]
    wallet: String,
    /// Chain name to get history
    #[arg(short, long)]
    chain: String,
}

#[instrument("history", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    for i in 0..3 {
        info!("Transaction {i}");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    Ok(())
}
