use std::{collections::HashMap, fs, path::PathBuf};

use clap::Parser;
use miette::{bail, Context, IntoDiagnostic};
use pallas::ledger::primitives::{conway::PlutusData, Fragment};
use tracing::instrument;

use crate::transaction::model::staging::{RedeemerPurpose, Redeemers};

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// utxo hash
    utxo_hash: String,
    /// utxo idx
    utxo_idx: usize,

    /// hex redeemer datum bytes
    #[arg(long, action)]
    hex: Option<String>,
    /// file path redeemer datum bytes
    #[arg(long, action)]
    file: Option<PathBuf>,
}

#[instrument("add redeemer spend", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let utxo_hash = args.utxo_hash.try_into().context("parsing utxo hash")?;
    let utxo_idx = args.utxo_idx;

    let redeemer_data_bytes = if let Some(hex) = args.hex {
        hex::decode(hex)
            .into_diagnostic()
            .context("parsing redeemer data hex to bytes")?
    } else if let Some(path) = args.file {
        if !path.exists() {
            bail!("redeemer data file path not exist")
        }
        fs::read(path).into_diagnostic()?
    } else {
        bail!("hex or file path is required");
    };

    let plutus_datum = PlutusData::decode_fragment(&redeemer_data_bytes)
        .map_err(|e| miette::ErrReport::msg(e.to_string()))
        .context("malformed redeemer datum")?;

    with_staging_tx(ctx, move |mut tx| {
        let redeemer_purpose = RedeemerPurpose::Spend(utxo_hash, utxo_idx);

        if let Some(redeemers) = tx.redeemers.as_mut() {
            redeemers.0.insert(redeemer_purpose, (plutus_datum, None));
        } else {
            let mut redeemers = HashMap::new();
            redeemers.insert(redeemer_purpose, (plutus_datum, None));
            tx.redeemers = Some(Redeemers(redeemers))
        }

        Ok(tx)
    })
    .await
}
