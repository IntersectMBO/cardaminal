use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// transaction id
    tx_id: String,
}

#[instrument("build", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
