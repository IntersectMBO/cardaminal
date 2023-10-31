use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// chain name
    chain: String,
}

#[instrument("create", skip_all, fields(chain=args.chain))]
pub async fn run(args: Args) -> miette::Result<()> {
    Ok(())
}
