use clap::Parser;
use miette::bail;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {}

#[instrument("list", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    bail!("error updating chain");
}
