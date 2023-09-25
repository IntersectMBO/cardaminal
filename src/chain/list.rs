use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {}

#[instrument("list", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    for i in 0..3 {
        info!(chain = i, "config");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    Ok(())
}
