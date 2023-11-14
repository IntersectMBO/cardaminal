use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::instrument;

use crate::transaction::model::Hash28;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// public key hash
    public_key_hash: String,
}

#[instrument("remove disclosed signer", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let hash: Hash28 = hex::decode(args.public_key_hash)
        .into_diagnostic()
        .context("parsing public key hash hex")?
        .try_into()?;

    with_staging_tx(ctx, move |mut tx| {
        if let Some(disclosed_signers) = tx.disclosed_signers {
            let disclosed_signers = disclosed_signers
                .into_iter()
                .filter(|s| !s.eq(&hash))
                .collect::<Vec<Hash28>>();

            tx.disclosed_signers = (!disclosed_signers.is_empty()).then_some(disclosed_signers);
        }

        Ok(tx)
    })
    .await
}
