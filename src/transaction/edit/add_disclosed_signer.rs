use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// public key hash required for signing the transaction
    public_key_hash: String,
}

#[instrument("add disclosed signer", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let hash: [u8; 28] = hex::decode(args.public_key_hash)
        .into_diagnostic()
        .context("parsing pubkeyhash hex to bytes")?
        .try_into()
        .map_err(|_| miette!("pubkeyhash incorrect length"))?;

    with_staging_tx(ctx, move |tx| Ok(tx.disclosed_signer(hash.into()))).await
}
