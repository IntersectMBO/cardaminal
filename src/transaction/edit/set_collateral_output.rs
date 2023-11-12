use clap::Parser;
use miette::{Context, IntoDiagnostic};
use pallas::ledger::addresses::Address as PallasAddress;
use tracing::instrument;

use crate::transaction::model::staging::{Address, CollateralOutput};

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// address to output collateral
    address: String,
    /// collateral lovelace
    lovelace: u64,
}

#[instrument("set_collateral_output", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let address: Address = PallasAddress::from_bech32(&args.address)
        .into_diagnostic()
        .context("parsing address")?
        .into();

    let lovelace = args.lovelace;

    with_staging_tx(ctx, move |mut tx| {
        let new = CollateralOutput { address, lovelace };

        tx.collateral_output = Some(new);

        tx
    })
    .await
}
