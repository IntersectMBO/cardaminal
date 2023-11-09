use std::{collections::HashMap, fs, path::PathBuf};

use clap::Parser;
use miette::{bail, Context, IntoDiagnostic};
use pallas::ledger::{
    primitives::{babbage::PlutusData, Fragment},
    traverse::ComputeHash,
};
use tracing::instrument;

use crate::transaction::model::Bytes;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// hex datum bytes
    #[arg(long, action)]
    hex: Option<String>,
    /// file path datum bytes
    #[arg(long, action)]
    file: Option<PathBuf>,
}

#[instrument("add datum", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let datum_bytes = if let Some(hex) = args.hex {
        hex::decode(hex)
            .into_diagnostic()
            .context("parsing datum hex to bytes")?
    } else if let Some(path) = args.file {
        if !path.exists() {
            bail!("datum file path not exist")
        }
        fs::read(path).into_diagnostic()?
    } else {
        bail!("hex or file path is required");
    };

    let plutus_datum = PlutusData::decode_fragment(&datum_bytes)
        .map_err(|e| miette::ErrReport::msg(e.to_string()))
        .context("datum malformed")?;

    let datum_hash = plutus_datum.compute_hash();

    with_staging_tx(ctx, move |mut tx| {
        if let Some(datums) = tx.datums.as_mut() {
            datums.insert(datum_hash.into(), Bytes(datum_bytes));
        } else {
            let mut datums = HashMap::new();
            datums.insert(datum_hash.into(), Bytes(datum_bytes));
            tx.datums = Some(datums)
        }

        Ok(tx)
    })
    .await
}
