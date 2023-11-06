use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// transaction id
    tx_id: String,
    /// utxo hash
    utxo_hash: String,
    /// utxo idx
    utxo_idx: u16,
}

#[instrument("remove", skip_all, fields())]
pub async fn run(_args: Args, _ctx: &crate::Context) -> miette::Result<()> {
    Ok(())
}
