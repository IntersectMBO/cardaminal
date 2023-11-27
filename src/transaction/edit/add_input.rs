use clap::Parser;
use miette::{bail, Context};
use tracing::instrument;

use crate::transaction::{edit::common::with_staging_tx, model::staging::Input};

#[derive(Parser)]
pub struct Args {
    /// utxo to use [hash]#[index]
    utxo: String,
}

#[instrument("add input", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let utxo: Input = args.utxo.parse().context("parsing utxo hash")?;

    with_staging_tx(ctx, move |mut tx| {
        let mut inputs = tx.inputs.unwrap_or_default();

        if inputs.iter().any(|i| i.eq(&utxo)) {
            bail!("input already added")
        }

        inputs.push(utxo);

        tx.inputs = Some(inputs);

        Ok(tx)
    })
    .await
}
