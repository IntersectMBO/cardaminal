use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {}

#[instrument("list", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    for i in 0..3 {
        info!("chain config {i}");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    Ok(())
}
