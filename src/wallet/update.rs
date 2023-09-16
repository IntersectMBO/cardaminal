use clap::Parser;
use miette::bail;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    chain: String,
}

#[instrument("update", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("starting wallet update");

    for i in 0..3 {
        info!("doing something {i}");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    bail!("error updating wallet");
}
