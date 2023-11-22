use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use pallas::txbuilder::Input;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// utxo hash
    utxo_hash: String,
    /// utxo idx
    utxo_idx: u64,
}

#[instrument("remove redeemer spend", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let utxo_hash: [u8; 32] = hex::decode(args.utxo_hash)
        .into_diagnostic()
        .context("parsing datum hex to bytes")?
        .try_into()
        .map_err(|_| miette!("utxo hash incorrect length"))?;

    let utxo_idx = args.utxo_idx;

    with_staging_tx(ctx, move |tx| {
        Ok(tx.remove_spend_redeemer(Input::new(utxo_hash.into(), utxo_idx)))
    })
    .await
}
