use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tracing::{info, instrument};

use crate::{
    transaction::model::staging::{Input, StagingTransaction},
    wallet::{config::Wallet, dal::WalletDB},
};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,
    /// transaction id
    id: i32,
    /// utxo hash
    utxo_hash: String,
    /// utxo idx
    utxo_idx: usize,
}

#[instrument("add", skip_all, fields())]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let utxo_hash = args.utxo_hash.try_into().context("parsing utxo hash")?;
    let utxo_idx = args.utxo_idx;

    crate::transaction::common::with_staging_tx(&args.wallet, args.id, ctx, move |mut tx| {
        let mut inputs = tx.inputs.unwrap_or(vec![]);

        let input = Input::new(utxo_hash, utxo_idx);
        inputs.push(input);

        tx.inputs = Some(inputs);

        Ok(tx)
    })
    .await
}
