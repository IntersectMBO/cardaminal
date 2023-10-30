use clap::Parser;
use tracing::instrument;

use super::RedeemerAction;

#[derive(Parser)]
pub struct Args {
    /// action to remove redeemer
    action: RedeemerAction,
    /// transaction id
    tx_id: String,
    /// policy id
    policy_id: String,
}

#[instrument("remove", skip_all, fields())]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
