use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// transaction id
    tx_id: String,

    /// datum bytes
    #[arg(long, short, action)]
    bytes: Option<Vec<u8>>,
    /// datum file path
    #[arg(long, short, action)]
    file: Option<String>,
}

#[instrument("add", skip_all, fields())]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
