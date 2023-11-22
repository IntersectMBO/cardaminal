use clap::Parser;
// use miette::{Context, IntoDiagnostic};
use tracing::instrument;

// use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// hex public key hash
    public_key: String,
    /// hex signature for a witness from the transaction
    signature: String,
}

#[instrument("add signature", skip_all, fields(args))]
pub async fn run(_args: Args, _ctx: &super::EditContext<'_>) -> miette::Result<()> {
    // TODO: This should operate on a BuiltTransaction not a StagingTransaction
    todo!()

    // let public_key: PublicKey = hex::decode(args.public_key)
    //     .into_diagnostic()
    //     .context("parsing public key hex")?
    //     .into();

    // let signature: Signature = hex::decode(args.signature)
    //     .into_diagnostic()
    //     .context("parsing signature hex")?
    //     .into();

    // TODO: verify if is possible to validate public key and signature
    // with_staging_tx(ctx, move |mut tx| {
    // if let Some(signatures) = tx.signatures.as_mut() {
    //     signatures.insert(public_key, signature);
    // } else {
    //     let mut signatures = HashMap::new();
    //     signatures.insert(public_key, signature);
    //     tx.signatures = Some(signatures)
    // }

    // Ok(tx)
    // })
    // .await
}
