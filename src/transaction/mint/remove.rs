use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// transaction id
    tx_id: String,
    /// mint asset [policy][name]
    asset: String,
}

#[instrument("remove", skip_all, fields())]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
