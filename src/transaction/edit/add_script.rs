use std::{collections::HashMap, fs, path::PathBuf};

use clap::{Parser, ValueEnum};
use miette::{bail, Context, IntoDiagnostic};
use pallas::{
    codec::minicbor,
    ledger::{primitives::conway::NativeScript, traverse::ComputeHash},
    txbuilder::plutus_script::PlutusScript,
};
use tracing::instrument;

use crate::transaction::model::{
    staging::{Script, ScriptKind},
    Hash28,
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
    let script_bytes = if let Some(hex) = args.hex {
        hex::decode(hex)
            .into_diagnostic()
            .context("parsing script hex to bytes")?
    } else if let Some(path) = args.file {
        if !path.exists() {
            bail!("script file path not exist")
        }
        fs::read(path).into_diagnostic()?
    } else {
        bail!("hex or file path is required");
    };

    let script_hash = match args.kind {
        Kind::Native => {
            let native_script: NativeScript = minicbor::decode(&script_bytes)
                .into_diagnostic()
                .context("parsing bytes to native script")?;
            native_script.compute_hash()
        }
        Kind::PlutusV1 => PlutusScript::v1()
            .from_bytes(script_bytes.clone())
            .build()
            .compute_hash(),
        Kind::PlutusV2 => PlutusScript::v2()
            .from_bytes(script_bytes.clone())
            .build()
            .compute_hash(),
    };

    let script_hash: Hash28 = script_hash.to_vec().try_into()?;

    with_staging_tx(ctx, move |mut tx| {
        let script = Script::new(args.kind.into(), script_bytes.into());
        if let Some(scripts) = tx.scripts.as_mut() {
            scripts.insert(script_hash, script);
        } else {
            let mut scripts = HashMap::new();
            scripts.insert(script_hash, script);
            tx.scripts = Some(scripts)
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
