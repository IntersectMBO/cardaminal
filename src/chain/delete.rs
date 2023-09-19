use clap::Parser;
use miette::bail;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to delete
    #[arg(short, long)]
    name: String,
}

#[instrument("delete", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("Deleting chain {}", args.name);

    for i in 0..3 {
        info!("Deleting data for the wallet: {i}");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    info!("Chain deleted");

    Ok(())
}
