use clap::Parser;
use comfy_table::Table;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {}

#[instrument("list", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    let mut table = Table::new();

    table
        .set_header(vec!["name", "chain", "slot", "utxos", "txs"])
        .add_row(vec!["my-wallet-1", "mainnet", "1234596", "12", "145"])
        .add_row(vec!["my-wallet-2", "mainnet", "1234588", "8", "23"])
        .add_row(vec!["my-wallet-3", "preprod", "1234", "1", "5"]);

    println!("{table}");

    Ok(())
}
