use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::instrument;

use crate::transaction::model::Hash32;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// hex datum hash
    datum_hash: String,
}

#[instrument("remove datum", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let datum_hash: Hash32 = hex::decode(args.datum_hash)
        .into_diagnostic()
        .context("parsing datum hash hex")?
        .try_into()?;

    with_staging_tx(ctx, move |mut tx| {
        if let Some(mut datums) = tx.datums {
            datums.remove(&datum_hash);

            tx.datums = (!datums.is_empty()).then_some(datums);
        }

        Ok(tx)
    })
    .await
}
