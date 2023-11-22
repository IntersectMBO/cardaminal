use clap::Parser;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {}

#[instrument("clear signer amount", skip_all)]
pub async fn run(_args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    with_staging_tx(ctx, move |tx| Ok(tx.clear_signature_amount_override())).await
}
