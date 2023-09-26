use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Wallet name to history update
    wallet: String,
}

#[instrument("update", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!(wallet = args.wallet, "updating");

    for i in 0..3 {
        info!(transaction = i, "update");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    info!("wallet updated");

    Ok(())
}
