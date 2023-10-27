use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// transaction id
    tx_id: String,
    /// utxo hash
    utxo_hash: String,
    /// utxo idx
    utxo_idx: String,
}

#[instrument("add", skip_all, fields())]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
