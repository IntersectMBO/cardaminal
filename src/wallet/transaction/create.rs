use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// A wallet address
    address: String,
    /// Value to transfer
    value: i64,
}

#[instrument("create", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    // TODO: check all parameters available
    info!("Transferred {} to {}", args.value, args.address);
    Ok(())
}
