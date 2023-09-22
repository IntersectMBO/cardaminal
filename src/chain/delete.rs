use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to delete
    name: String,
    /// automatically detached the wallets that are attached to this chain if any
    #[arg(long, default_value_t)]
    detached: bool,
}

#[instrument("delete", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("Deleting chain {}, {}", args.name, args.detached);

    for i in 0..3 {
        info!("Deleting data for the wallet: {i}");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    info!("Chain deleted");

    Ok(())
}
