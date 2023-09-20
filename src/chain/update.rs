use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to delete
    #[arg(short, long)]
    name: String,
}

#[instrument("update", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("Updating chain {}", args.name);

    for i in 0..3 {
        info!("Chain update {i}");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    info!("Chain updated");

    Ok(())
}
