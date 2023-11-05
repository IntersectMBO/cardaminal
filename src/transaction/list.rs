use clap::Parser;
use miette::IntoDiagnostic;
use sea_orm::Order;
use tracing::instrument;

use crate::{
    utils::OutputFormatter,
    wallet::{config::Wallet, dal::WalletDB},
    OutputFormat,
};

use super::config::TransactionView;

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,

    /// Number of page to find
    #[arg(short, long, default_value_t = 0)]
    page: u64,

    /// Number of transaction per page
    #[arg(short, long, default_value_t = 20)]
    size: u64,
}

#[instrument("list", skip_all, fields(wallet=args.wallet))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let transactions = wallet_db
        .paginate_transactions(Order::Asc, Some(args.size))
        .fetch_page(args.page)
        .await
        .into_diagnostic()?;

    let transactions_view = transactions
        .iter()
        .map(|transaction| transaction.clone().into())
        .collect::<Vec<TransactionView>>();

    match ctx.output_format {
        OutputFormat::Json => transactions_view.to_json(),
        OutputFormat::Table => transactions_view.to_table(),
    }

    Ok(())
}
