use std::{fs, path::PathBuf};

use clap::{Parser, ValueEnum};
use miette::{bail, Context, IntoDiagnostic};
use tracing::instrument;

use crate::transaction::model::{
    staging::{Script, ScriptBytes, ScriptKind},
    Bytes,
};

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// type of script
    kind: Kind,
    /// hex script bytes
    #[arg(long, action)]
    hex: Option<String>,
    ///file path script bytes
    #[arg(long, action)]
    file: Option<PathBuf>,
}

#[instrument("add script", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let script_bytes: ScriptBytes = if let Some(hex) = args.hex {
        let bytes = hex::decode(hex)
            .into_diagnostic()
            .context("parsing script hex to bytes")?;
        Bytes(bytes)
    } else if let Some(path) = args.file {
        if !path.exists() {
            bail!("script file path not exist")
        }
        let bytes = fs::read(path).into_diagnostic()?;
        Bytes(bytes)
    } else {
        bail!("hex or file path is required");
    };

    with_staging_tx(ctx, move |mut tx| {
        let script = Script::new(args.kind.into(), script_bytes);

        if let Some(scripts) = tx.scripts.as_mut() {
            scripts.push(script)
        } else {
            tx.scripts = Some(vec![script])
        }

        Ok(tx)
    })
    .await
}

#[derive(ValueEnum, Clone)]
enum Kind {
    Native,
    PlutusV1,
    PlutusV2,
}
impl From<Kind> for ScriptKind {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Native => ScriptKind::Native,
            Kind::PlutusV1 => ScriptKind::PlutusV1,
            Kind::PlutusV2 => ScriptKind::PlutusV2,
        }
    }
}
