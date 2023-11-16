use std::{collections::HashMap, fs, path::PathBuf};

use clap::Parser;
use miette::{bail, Context, IntoDiagnostic};
use pallas::ledger::primitives::{conway::PlutusData, Fragment};
use tracing::instrument;

use crate::transaction::model::{
    staging::{RedeemerPurpose, Redeemers},
    Hash28,
};

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// Policy id hex
    policy_id: String,

    /// hex redeemer datum bytes
    #[arg(long, action)]
    hex: Option<String>,
    /// file path redeemer datum bytes
    #[arg(long, action)]
    file: Option<PathBuf>,
}

#[instrument("add redeemer mint", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let policy: Hash28 = hex::decode(args.policy_id)
        .into_diagnostic()
        .context("parsing policy hex")?
        .try_into()?;

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
        let redeemer_purpose = RedeemerPurpose::Mint(policy);

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
