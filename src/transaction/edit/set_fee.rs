use clap::Parser;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// fee in lovelace
    lovelace: u64,
}

#[instrument("set_fee", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let value = args.lovelace;

    with_staging_tx(ctx, move |mut tx| {
        tx.fee = Some(value);

        Ok(tx)
    })
    .await
}
