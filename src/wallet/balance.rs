use std::collections::HashMap;

use clap::Parser;
use miette::{Context, IntoDiagnostic};
use pallas::{
    crypto::hash::Hash,
    ledger::traverse::{Era, MultiEraOutput},
};
use sea_orm::Order;
use tracing::instrument;

use crate::{utils::OutputFormatter, OutputFormat};

use super::{
    config::{BalanceView, Wallet},
    dal::WalletDB,
};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet to query
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    name: Option<String>,
}

#[instrument("balance", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet_name = args
        .name
        .ok_or(miette::miette!("wallet param is required"))?;

    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &wallet_name)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let utxos = wallet_db
        .fetch_all_utxos(Order::Asc)
        .await
        .into_diagnostic()?;

    let mut lovelace: u64 = 0;

    let mut _tokens: HashMap<Hash<28>, HashMap<&[u8], u64>> = HashMap::new();

    for utxo in utxos.iter() {
        let era = Era::try_from(utxo.era)
            .into_diagnostic()
            .context("parsing era")?;

        let output = MultiEraOutput::decode(era, &utxo.cbor).into_diagnostic()?;
        lovelace += output.lovelace_amount();
    }

    let balance = BalanceView::new(lovelace);

    match ctx.output_format {
        OutputFormat::Json => balance.to_json(),
        OutputFormat::Table => balance.to_table(),
    }

    Ok(())
}
