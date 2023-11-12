use clap::Parser;
use miette::{Context, IntoDiagnostic};
use pallas::ledger::addresses::Address as PallasAddress;
use tracing::instrument;

use crate::transaction::model::staging::Address;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// address to return change values
    address: String,
}

#[instrument("set_change_address", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let address: Address = PallasAddress::from_bech32(&args.address)
        .into_diagnostic()
        .context("parsing address")?
        .into();

    with_staging_tx(ctx, move |mut tx| {
        tx.change_address = Some(address);

        tx
    })
    .await
}
