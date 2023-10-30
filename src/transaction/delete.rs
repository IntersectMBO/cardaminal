use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// transaction id
    tx_id: String,
}

#[instrument("delete", skip_all, fields(tx_id=args.tx_id))]
pub async fn run(args: Args) -> miette::Result<()> {
    Ok(())
}
