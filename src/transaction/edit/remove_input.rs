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

#[instrument("remove input", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let utxo_hash = args.utxo_hash.try_into().context("parsing utxo hash")?;
    let utxo_idx = args.utxo_idx;

    with_staging_tx(ctx, move |mut tx| {
        if let Some(inputs) = tx.inputs {
            let inputs = inputs
                .into_iter()
                .filter(|i| !(i.tx_hash.eq(&utxo_hash) && i.tx_index.eq(&utxo_idx)))
                .collect::<Vec<Input>>();

            tx.inputs = (!inputs.is_empty()).then_some(inputs);
        }

        tx
    })
    .await
}
