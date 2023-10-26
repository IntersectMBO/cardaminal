use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {}

#[instrument("inspect", skip_all, fields())]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
