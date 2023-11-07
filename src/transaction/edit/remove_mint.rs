use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::instrument;

use crate::transaction::model::Hash28;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// mint asset policy
    policy: String,
    /// mint asset name
    asset: String,
}

#[instrument("remove mint", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let policy = hex::decode(args.policy)
        .into_diagnostic()
        .context("parsing policy hex")?;
    let policy = Hash28(policy.try_into().unwrap());

    let asset = hex::decode(args.asset)
        .into_diagnostic()
        .context("parsing name hex")?;
    let asset = crate::transaction::model::Bytes(asset);

    with_staging_tx(ctx, move |mut tx| {
        if let Some(mut mint_assets) = tx.mint.clone() {
            if let Some(assets) = mint_assets.0.get_mut(&policy) {
                assets.remove(&asset);
                if assets.is_empty() {
                    mint_assets.0.remove(&policy);
                }
            }

            tx.mint = (!mint_assets.0.is_empty()).then_some(mint_assets);
        }

        tx
    })
    .await
}
