use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Wallet name
    #[arg(short, long)]
    name: String,
}

#[instrument("create", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("Wallet {} created", args.name);
    Ok(())
}
