use clap::Parser;
use miette::IntoDiagnostic;
use sea_orm::Order;
use tracing::instrument;

use crate::{
    utils::OutputFormatter,
    wallet::{
        config::{UtxoView, Wallet},
        dal::WalletDB,
    },
    OutputFormat,
};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet to query
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    name: Option<String>,

    /// Number of page to find
    #[arg(short, long, default_value_t = 0)]
    page: u64,

    /// Number of utxo per page
    #[arg(short, long, default_value_t = 20)]
    size: u64,
}

#[instrument("utxos", skip_all)]
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
        .paginate_utxos(Order::Asc, Some(args.size))
        .fetch_page(args.page)
        .await
        .into_diagnostic()?;

    let utxos_view = utxos
        .iter()
        .map(|utxo| utxo.clone().try_into())
        .collect::<Result<Vec<UtxoView>, _>>()?;

    match ctx.output_format {
        OutputFormat::Json => utxos_view.to_json(),
        OutputFormat::Table => utxos_view.to_table(),
    }

    Ok(())
}
