use clap::Parser;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// output index
    output_index: usize,
}

#[instrument("remove output", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    with_staging_tx(ctx, move |mut tx| {
        if let Some(mut outputs) = tx.outputs.clone() {
            outputs.remove(args.output_index);
            tx.outputs = (!outputs.is_empty()).then_some(outputs);
        }

        tx
    })
    .await
}
