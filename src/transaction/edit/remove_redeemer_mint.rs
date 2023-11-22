use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// policy id
    policy: String,
}

#[instrument("remove redeemer mint", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let policy: [u8; 28] = hex::decode(args.policy)
        .into_diagnostic()
        .context("parsing policy hex")?
        .try_into()
        .map_err(|_| miette!("policy id incorrect length"))?;

    with_staging_tx(ctx, move |tx| Ok(tx.remove_mint_redeemer(policy.into()))).await
}
