use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// script hash
    script_hash: String,
}

#[instrument("remove script", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let script_hash: [u8; 28] = hex::decode(args.script_hash)
        .into_diagnostic()
        .context("parsing script hash hex")?
        .try_into()
        .map_err(|_| miette!("script hash incorrect length"))?;

    with_staging_tx(ctx, move |tx| {
        Ok(tx.remove_script_by_hash(script_hash.into()))
    })
    .await
}
