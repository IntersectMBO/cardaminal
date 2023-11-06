use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::instrument;

use crate::transaction::{
    edit::common::with_staging_tx,
    model::staging::{Address, Output},
};
use pallas::ledger::addresses::Address as PallasAddress;

#[derive(Parser)]
pub struct Args {
    /// output address
    address: String,

    /// amount of lovelace to include
    lovelace_amount: Option<u64>,

    /// output assets [policy][name]:[amount]
    assets: Vec<String>,

    /// datum hash
    #[arg(long, action)]
    datum: Option<String>,

    /// datum file path
    #[arg(long, action)]
    datum_file: Option<String>,

    /// reference script hash
    #[arg(long, action)]
    reference_script: Option<String>,

    /// reference script file path
    #[arg(long, action)]
    reference_script_file: Option<String>,
}

// TODO: find value from params
const MIN_UTXO: u64 = 1_000_000;

#[instrument("add", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let address: Address = PallasAddress::from_bech32(&args.address)
        .into_diagnostic()
        .context("parsing address")?
        .into();

    let lovelace = args.lovelace_amount.unwrap_or(MIN_UTXO);

    with_staging_tx(ctx, move |mut tx| {
        let mut outputs = tx.outputs.unwrap_or(vec![]);

        let new = Output {
            address,
            lovelace,
            assets: None,
            datum: None,
            script: None,
        };

        outputs.push(new);

        tx.outputs = Some(outputs);

        tx
    })
    .await
}
