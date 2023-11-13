use std::collections::HashMap;

use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::instrument;

use super::common::with_staging_tx;
use crate::transaction::model::{staging::MintAssets, Bytes, Hash28};

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
    let policy: Hash28 = hex::decode(args.policy)
        .into_diagnostic()
        .context("parsing policy hex")?
        .try_into()?;

    let asset: Bytes = hex::decode(args.asset)
        .into_diagnostic()
        .context("parsing name hex")?
        .into();

    let amount = args.amount;

    with_staging_tx(ctx, move |mut tx| {
        let mut mints = tx.mint.unwrap_or(MintAssets(HashMap::new()));

        let mut assets = mints.0.remove(&policy).unwrap_or_default();
        assets.insert(asset, amount);

        mints.0.insert(policy, assets);

        tx.mint = Some(mints);

        Ok(tx)
    })
    .await
}
