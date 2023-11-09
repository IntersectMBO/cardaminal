use std::collections::HashMap;

use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::instrument;

use super::common::with_staging_tx;
use crate::transaction::model::{staging::MintAssets, Hash28};

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
    let policy = hex::decode(args.policy)
        .into_diagnostic()
        .context("parsing policy hex")?;

    let policy = Hash28(policy.try_into().unwrap());

    let asset = hex::decode(args.asset)
        .into_diagnostic()
        .context("parsing name hex")?;

    let asset = crate::transaction::model::Bytes(asset);

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
