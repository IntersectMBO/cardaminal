use clap::Parser;
use comfy_table::Table;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// name of the wallet to query
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: Option<String>,
}

#[instrument("utxos", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    let mut table = Table::new();

    table
        .set_header(vec!["slot", "tx", "idx", "assets", "datum"])
        .add_row(vec!["123456", "abcdabcdabcd", "0", "24 ADA + 20 HOSKY", ""])
        .add_row(vec!["123589", "abcdabcdabcd", "1", "40 ADA", "abcdabcd"])
        .add_row(vec!["123763", "abcdabcdabcd", "0", "8 ADA", ""]);

    println!("{table}");

    Ok(())
}
