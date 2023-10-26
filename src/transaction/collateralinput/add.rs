use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {}

#[instrument("add", skip_all, fields())]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
