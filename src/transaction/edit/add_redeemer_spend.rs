use std::{fs, path::PathBuf};

use clap::Parser;
use miette::{bail, miette, Context, IntoDiagnostic};
use pallas::txbuilder::Input;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// utxo hash
    utxo_hash: String,
    /// utxo idx
    utxo_idx: u64,

    /// hex redeemer datum bytes
    #[arg(long, action)]
    data_hex: Option<String>,
    /// file path redeemer datum bytes
    #[arg(long, action)]
    data_file: Option<PathBuf>,
    // TODO: Exunits
}

#[instrument("add redeemer spend", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let utxo_hash: [u8; 32] = hex::decode(args.utxo_hash)
        .into_diagnostic()
        .context("parsing datum hex to bytes")?
        .try_into()
        .map_err(|_| miette!("utxo hash incorrect length"))?;

    let utxo_idx = args.utxo_idx;

    let redeemer_data_bytes = if let Some(hex) = args.data_hex {
        hex::decode(hex)
            .into_diagnostic()
            .context("parsing redeemer data hex to bytes")?
    } else if let Some(path) = args.data_file {
        if !path.exists() {
            bail!("redeemer data file path not exist")
        }
        fs::read(path).into_diagnostic()?
    } else {
        bail!("hex or file path is required");
    };

    with_staging_tx(ctx, move |tx| {
        Ok(tx.add_spend_redeemer(
            Input::new(utxo_hash.into(), utxo_idx),
            redeemer_data_bytes,
            None,
        ))
    })
    .await
}
