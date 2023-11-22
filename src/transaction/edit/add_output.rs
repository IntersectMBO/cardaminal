use std::{fs, path::PathBuf};

use clap::Parser;
use miette::{bail, Context, IntoDiagnostic};
use pallas::txbuilder::Output;
use tracing::instrument;

use crate::transaction::edit::common::with_staging_tx;
use pallas::ledger::addresses::Address;

#[derive(Parser)]
pub struct Args {
    /// output address
    address: String,

    /// amount of lovelace to include
    lovelace_amount: u64,

    /// output assets [policy]:[name]:[amount]
    #[arg(short, long, action)]
    assets: Option<Vec<String>>,

    /// datum via hex string
    #[arg(long, action)]
    datum: Option<String>,

    /// datum via file path
    #[arg(long, action)]
    datum_file: Option<PathBuf>,

    /// reference script hash
    #[arg(long, action)]
    reference_script: Option<String>,

    /// reference script file path
    #[arg(long, action)]
    reference_script_file: Option<PathBuf>,
}

#[instrument("add", skip_all, fields())]
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

    /*
       TODO accept datum hash OR inline datum. right now we will assume inline

       --datum <"inline" | "hash"> <BYTES>, OR
       --datum-file <"inline" | "hash"> <FILE>
    */

    match (args.datum, args.datum_file) {
        (Some(_), Some(_)) => bail!("datum must be specified by hex string OR by file, not both"),
        (Some(d), _) => {
            let data = hex::decode(d)
                .into_diagnostic()
                .context("parsing datum hex to bytes")?;

            output = output.set_inline_datum(data);
        }
        (_, Some(path)) => {
            if !path.exists() {
                bail!("datum file path doesn't exist")
            }
            let data = fs::read(path).into_diagnostic()?;

            output = output.set_inline_datum(data);
        }
        _ => (),
    };

    if args.reference_script.is_some() || args.reference_script_file.is_some() {
        todo!("reference scripts not yet supported") // TODO
    }

    with_staging_tx(ctx, move |tx| Ok(tx.output(output))).await
}
