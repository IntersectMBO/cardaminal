use clap::Parser;
use miette::{bail, miette, Context, IntoDiagnostic};
use pallas::txbuilder::Input;
use tracing::instrument;

use crate::transaction::edit::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// utxo to use [hash]#[index]
    utxo: String,
}

#[instrument("add input", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let mut parts = args.utxo.split('#').collect::<Vec<_>>();

    if parts.len() != 2 {
        bail!("invalid utxo string");
    }

    let utxo_hash: [u8; 32] = hex::decode(parts.remove(0).to_owned())
        .into_diagnostic()
        .context("parsing datum hex to bytes")?
        .try_into()
        .map_err(|_| miette!("utxo hash incorrect length"))?;

    let utxo_idx = parts.remove(0).parse().into_diagnostic()?;

    with_staging_tx(ctx, move |tx| {
        Ok(tx.input(Input::new(utxo_hash.into(), utxo_idx)))
    })
    .await
}
