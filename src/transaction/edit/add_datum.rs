use std::{fs, path::PathBuf};

use clap::Parser;
use miette::{bail, Context, IntoDiagnostic};
use tracing::instrument;

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
            bail!("datum file path doesn't exist")
        }
        fs::read(path).into_diagnostic()?
    } else {
        bail!("hex or file path is required");
    };

    with_staging_tx(ctx, move |tx| Ok(tx.datum(datum_bytes))).await
}
