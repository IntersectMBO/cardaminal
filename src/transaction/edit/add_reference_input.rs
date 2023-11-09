use clap::Parser;
use miette::{bail, Context};
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

#[instrument("add reference input", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let utxo_hash = args.utxo_hash.try_into().context("parsing utxo hash")?;
    let utxo_idx = args.utxo_idx;

    with_staging_tx(ctx, move |mut tx| {
        let mut inputs = tx.reference_inputs.unwrap_or(vec![]);

        if inputs
            .iter()
            .any(|i| i.tx_hash.eq(&utxo_hash) && i.tx_index.eq(&utxo_idx))
        {
            bail!("reference input already added")
        }

        let input = Input::new(utxo_hash, utxo_idx);
        inputs.push(input);

        tx.reference_inputs = Some(inputs);

        Ok(tx)
    })
    .await
}
