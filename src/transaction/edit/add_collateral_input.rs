use clap::Parser;
use miette::Context;
use tracing::instrument;

use crate::transaction::model::staging::Input;

use super::common::with_staging_tx;

#[derive(Parser)]
pub struct Args {
    /// utxo hash
    utxo_hash: String,

    /// utxo idx
    utxo_idx: usize,
}

#[instrument("add_collateral_input", skip_all, fields())]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let utxo_hash = args.utxo_hash.try_into().context("parsing utxo hash")?;
    let utxo_idx = args.utxo_idx;

    with_staging_tx(ctx, move |mut tx| {
        let mut inputs = tx.collateral_inputs.unwrap_or(vec![]);

        let input = Input::new(utxo_hash, utxo_idx);
        inputs.push(input);

        tx.collateral_inputs = Some(inputs);

        Ok(tx)
    })
    .await
}
