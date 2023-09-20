use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// A wallet address
    #[arg(short, long)]
    address: String,
    /// Value to transfer
    #[arg(short, long)]
    value: i64,
}

#[instrument("create", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("Transferred {} to {}", args.value, args.address);
    Ok(())
}
