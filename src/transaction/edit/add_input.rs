use clap::Parser;
use miette::{miette, Context, IntoDiagnostic};
use pallas::txbuilder::Input;
use tracing::instrument;

use crate::transaction::edit::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// utxo hash
    utxo_hash: String,

    /// utxo idx
    utxo_idx: u64,
}

#[instrument("add input", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let utxo_hash: [u8; 32] = hex::decode(args.utxo_hash)
        .into_diagnostic()
        .context("parsing datum hex to bytes")?
        .try_into()
        .map_err(|_| miette!("utxo hash incorrect length"))?;

    let utxo_idx = args.utxo_idx;

    with_staging_tx(ctx, move |tx| {
        Ok(tx.input(Input::new(utxo_hash.into(), utxo_idx)))
    })
    .await
}
