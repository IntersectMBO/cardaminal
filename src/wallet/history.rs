use clap::Parser;
use comfy_table::Table;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// name of the wallet to query
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: Option<String>,
}

#[instrument("history", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    let mut table = Table::new();

    table
        .set_header(vec!["slot", "tx", "idx", "change"])
        .add_row(vec!["123456", "abcdabcdabcd", "0", "+24 ADA"])
        .add_row(vec!["123589", "abcdabcdabcd", "1", "-4 ADA"])
        .add_row(vec!["123763", "abcdabcdabcd", "0", "+200 ADA"]);

    println!("{table}");

    Ok(())
}
