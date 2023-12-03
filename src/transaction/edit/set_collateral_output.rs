use clap::Parser;
use miette::{Context, IntoDiagnostic};
use pallas::{ledger::addresses::Address, txbuilder::Output};
use tracing::instrument;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// output address
    address: String,

    /// amount of lovelace to include
    lovelace_amount: u64,

    /// output assets [policy]:[name]:[amount]
    #[arg(short, long, action)]
    assets: Option<Vec<String>>,
}

#[instrument("set_collateral_output", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let address = Address::from_bech32(&args.address)
        .into_diagnostic()
        .context("parsing address")?;

    let lovelace = args.lovelace_amount;

    let mut output = Output::new(address, lovelace);

    if let Some(value) = args.assets {
        for asset_string in value {
            let parts = asset_string.split(':').collect::<Vec<&str>>();
            if parts.len() != 3 {
                return Err(miette::ErrReport::msg("invalid asset string format"));
            }

            let policy: [u8; 28] = hex::decode(parts[0])
                .into_diagnostic()?
                .try_into()
                .map_err(|_| miette::miette!("incorrect size for hash 28"))?;

            let name = hex::decode(parts[1])
                .into_diagnostic()
                .context("parsing name hex")?;

            let amount = parts[2]
                .parse::<u64>()
                .into_diagnostic()
                .context("parsing amount u64")?;

            output = output
                .add_asset(policy.into(), name, amount)
                .into_diagnostic()?;
        }
    }

    with_staging_tx(ctx, move |tx| Ok(tx.collateral_output(output))).await
}
