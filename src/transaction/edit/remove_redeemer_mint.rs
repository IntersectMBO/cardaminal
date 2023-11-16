use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::instrument;

use crate::transaction::model::{staging::RedeemerPurpose, Hash28};

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// policy id
    policy_id: String,
}

#[instrument("remove redeemer mint", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let policy: Hash28 = hex::decode(args.policy_id)
        .into_diagnostic()
        .context("parsing policy hex")?
        .try_into()?;

    with_staging_tx(ctx, move |mut tx| {
        if let Some(mut redeemers) = tx.redeemers {
            let redeemer_purpose = RedeemerPurpose::Mint(policy);

            redeemers.0.remove(&redeemer_purpose);

            tx.redeemers = (!redeemers.0.is_empty()).then_some(redeemers);
        }

        Ok(tx)
    })
    .await
}
