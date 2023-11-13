use clap::Parser;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// slot to validate ttl
    slot: u64,
}

#[instrument("set ttl", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    with_staging_tx(ctx, move |mut tx| {
        tx.invalid_from_slot = Some(args.slot);

        Ok(tx)
    })
    .await
}
