use std::{fs, path::PathBuf};

use clap::Parser;
use miette::{bail, Context, IntoDiagnostic};
use tracing::instrument;

use crate::transaction::model::{staging::DatumBytes, Bytes};

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

#[instrument("add datum", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let datum: DatumBytes = if let Some(hex) = args.hex {
        let bytes = hex::decode(hex)
            .into_diagnostic()
            .context("parsing datum hex to bytes")?;
        Bytes(bytes)
    } else if let Some(path) = args.file {
        if !path.exists() {
            bail!("datum file path not exist")
        }
        let bytes = fs::read(path).into_diagnostic()?;
        Bytes(bytes)
    } else {
        bail!("hex or file path is required");
    };

    with_staging_tx(ctx, move |mut tx| {
        if let Some(datums) = tx.datums.as_mut() {
            datums.push(datum)
        } else {
            tx.datums = Some(vec![datum])
        }

        tx
    })
    .await
}
