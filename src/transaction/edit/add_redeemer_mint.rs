use std::{fs, path::PathBuf};

use clap::Parser;
use miette::{bail, miette, Context, IntoDiagnostic};
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// Policy id hex
    policy_id: String,

    /// hex redeemer datum bytes
    #[arg(long, action)]
    data_hex: Option<String>,
    /// file path redeemer datum bytes
    #[arg(long, action)]
    data_file: Option<PathBuf>,
    // TODO: exunit budget
}

#[instrument("add redeemer mint", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let policy: [u8; 28] = hex::decode(args.policy_id)
        .into_diagnostic()
        .context("parsing policy hex")?
        .try_into()
        .map_err(|_| miette!("policy id incorrect length"))?;

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
        Ok(tx.add_mint_redeemer(policy.into(), redeemer_data_bytes, None))
    })
    .await
}
