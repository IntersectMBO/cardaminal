use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to update
    name: String,
}

#[instrument("update", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!(chain = args.name, "updating");

    for i in 0..3 {
        info!(slot = i, "update");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    info!("chain updated");

    Ok(())
}
