use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::instrument;

use crate::transaction::model::{staging::PublicKey, Bytes};

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// public key hash
    public_key: String,
}

#[instrument("remove signature", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let public_key: PublicKey = Bytes(
        hex::decode(args.public_key)
            .into_diagnostic()
            .context("parsing public key hex")?,
    );

    with_staging_tx(ctx, move |mut tx| {
        if let Some(mut signatures) = tx.signatures {
            signatures.remove(&public_key);

            tx.signatures = (!signatures.is_empty()).then_some(signatures);
        }

        Ok(tx)
    })
    .await
}
