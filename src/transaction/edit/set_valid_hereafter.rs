use clap::Parser;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// slot to validate
    slot: u64,
}

#[instrument("set valid hereafter", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    with_staging_tx(ctx, move |mut tx| {
        tx.valid_from_slot = Some(args.slot);
        tx
    })
    .await
}
