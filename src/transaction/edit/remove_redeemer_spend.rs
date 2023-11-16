use clap::Parser;
use miette::Context;
use tracing::instrument;

use crate::transaction::model::staging::RedeemerPurpose;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// utxo hash
    utxo_hash: String,
    /// utxo idx
    utxo_idx: usize,
}

#[instrument("remove redeemer spend", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let utxo_hash = args.utxo_hash.try_into().context("parsing utxo hash")?;
    let utxo_idx = args.utxo_idx;

    with_staging_tx(ctx, move |mut tx| {
        if let Some(mut redeemers) = tx.redeemers {
            let redeemer_purpose = RedeemerPurpose::Spend(utxo_hash, utxo_idx);

            redeemers.0.remove(&redeemer_purpose);

            tx.redeemers = (!redeemers.0.is_empty()).then_some(redeemers);
        }

        Ok(tx)
    })
    .await
}
