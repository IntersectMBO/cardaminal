use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// mint asset policy
    policy: String,
    /// mint asset name
    asset: String,
    /// mint asset amount
    amount: i64,
}

#[instrument("add_mint", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let policy: [u8; 28] = hex::decode(args.policy)
        .into_diagnostic()
        .context("parsing policy hex")?
        .try_into()
        .map_err(|_| miette!("policy id incorrect length"))?;

    let asset = hex::decode(args.asset)
        .into_diagnostic()
        .context("parsing name hex")?;

    let amount = args.amount;

    with_staging_tx(ctx, move |tx| {
        Ok(tx
            .mint_asset(policy.into(), asset, amount)
            .into_diagnostic()?)
    })
    .await
}
