use clap::Parser;
use miette::{Context, IntoDiagnostic, bail};
use tracing::instrument;

use crate::transaction::model::Hash28;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// public key hash required for signing the transaction
    public_key_hash: String,
}

#[instrument("add disclosed signer", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let hash_bytes = hex::decode(args.public_key_hash)
        .into_diagnostic()
        .context("parsing public key hash hex")?;

    let hash = Hash28(
        hash_bytes
            .as_slice()
            .try_into()
            .into_diagnostic()
            .context("public key hash malformed")?,
    );

    with_staging_tx(ctx, move |mut tx| {
        let mut disclosed_signers = tx.disclosed_signers.unwrap_or(vec![]);

        if disclosed_signers.iter().any(|s| s.eq(&hash)) {
            bail!("disclosed signer already added")
        }

        disclosed_signers.push(hash);

        tx.disclosed_signers = Some(disclosed_signers);
        Ok(tx)
    })
    .await
}
