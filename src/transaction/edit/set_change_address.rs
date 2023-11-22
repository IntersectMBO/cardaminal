use clap::Parser;
use miette::{Context, IntoDiagnostic};
use pallas::ledger::addresses::Address;
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// address to return change values
    address: String,
}

#[instrument("set_change_address", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let address = Address::from_bech32(&args.address)
        .into_diagnostic()
        .context("parsing address")?;

    with_staging_tx(ctx, move |tx| Ok(tx.change_address(address))).await
}
