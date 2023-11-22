use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// hex datum hash
    datum_hash: String,
}

#[instrument("remove datum", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let datum_hash: [u8; 32] = hex::decode(args.datum_hash)
        .into_diagnostic()
        .context("parsing datum hash hex")?
        .try_into()
        .map_err(|_| miette!("datum hash incorrect length"))?;

    with_staging_tx(
        ctx,
        move |tx| Ok(tx.remove_datum_by_hash(datum_hash.into())),
    )
    .await
}
