use clap::Parser;
use miette::bail;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    chain: String,
}

#[instrument("create", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    bail!("error creating wallet");
}
