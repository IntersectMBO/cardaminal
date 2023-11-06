use clap::Parser;
use tracing::instrument;

use super::RedeemerAction;

#[derive(Parser)]
pub struct Args {
    /// action to apply redeemer
    action: RedeemerAction,
    /// transaction id
    tx_id: String,
    /// utxo hash
    utxo_hash: String,
    /// utxo idx
    utxo_idx: u16,
}

#[instrument("add", skip_all, fields())]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
