use std::collections::HashMap;

use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::instrument;

use crate::transaction::model::{
    staging::{PublicKey, Signature},
    Bytes,
};

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// hex public key hash
    public_key: String,
    /// hex signature for a witness from the transaction
    signature: String,
}

#[instrument("add signature", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let public_key: PublicKey = Bytes(
        hex::decode(args.public_key)
            .into_diagnostic()
            .context("parsing public key hex")?,
    );

    let signature: Signature = Bytes(
        hex::decode(args.signature)
            .into_diagnostic()
            .context("parsing signature hex")?,
    );

    // TODO: verify if is possible to validate public key and signature
    with_staging_tx(ctx, move |mut tx| {
        if let Some(signatures) = tx.signatures.as_mut() {
            signatures.insert(public_key, signature);
        } else {
            let mut signatures = HashMap::new();
            signatures.insert(public_key, signature);
            tx.signatures = Some(signatures)
        }

        Ok(tx)
    })
    .await
}
