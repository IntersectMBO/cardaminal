use clap::Parser;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// number of signers to calculate min fee
    number_of_signers: u8,
}

#[instrument("set signer amount", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    with_staging_tx(ctx, move |mut tx| {
        tx.signature_amount_override = Some(args.number_of_signers);

        Ok(tx)
    })
    .await
}
