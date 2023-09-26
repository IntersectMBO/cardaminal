use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// friendly name to identify the chain
    name: String,

    /// [host]:[port] of the upstream node
    #[arg(short, long)]
    upstream: Option<String>,

    /// network magic of the chain
    #[arg(short, long)]
    magic: Option<String>,

    /// [slot],[hash] of the sync start point
    #[arg(short, long)]
    after: Option<String>,
}

#[instrument("create", skip_all, fields(name=args.name))]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("creating chain");

    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    info!("chain created");

    Ok(())
}
