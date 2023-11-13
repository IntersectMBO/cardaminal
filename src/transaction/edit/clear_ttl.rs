use clap::Parser;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {}

#[instrument("clear ttl", skip_all)]
pub async fn run(_args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    with_staging_tx(ctx, move |mut tx| {
        tx.invalid_from_slot = None;

        Ok(tx)
    })
    .await
}
