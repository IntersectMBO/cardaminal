use clap::Parser;
// use miette::{Context, IntoDiagnostic};
use tracing::instrument;

// use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// public key hash
    public_key: String,
}

#[instrument("remove signature", skip_all, fields(args))]
pub async fn run(_args: Args, _ctx: &super::EditContext<'_>) -> miette::Result<()> {
    // TODO should operate on built tx not staging
    todo!();

    // let public_key: PublicKey = hex::decode(args.public_key)
    //     .into_diagnostic()
    //     .context("parsing public key hex")?
    //     .into();

    // with_staging_tx(ctx, move |mut tx| {
    //     if let Some(mut signatures) = tx.signatures {
    //         signatures.remove(&public_key);

    //         tx.signatures = (!signatures.is_empty()).then_some(signatures);
    //     }

    //     Ok(tx)
    // })
    // .await
}
