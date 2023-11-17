use clap::Parser;
use miette::{IntoDiagnostic, Context};
use tracing::instrument;

use crate::transaction::model::Hash28;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// script hash
    script_hash: String,
}

#[instrument("remove script", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let script_hash: Hash28 = hex::decode(args.script_hash)
        .into_diagnostic()
        .context("parsing script hash hex")?
        .try_into()?;

    with_staging_tx(ctx, move |mut tx| {
        if let Some(mut scripts) = tx.scripts {
            scripts.remove(&script_hash);

            tx.scripts = (!scripts.is_empty()).then_some(scripts);
        }

        Ok(tx)
    })
    .await
}
