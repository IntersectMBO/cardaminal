use clap::Parser;
use comfy_table::Table;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {}

#[instrument("list", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    let mut table = Table::new();

    table
        .set_header(vec!["name", "upstream", "slot", "wallets"])
        .add_row(vec!["mainnet", "some-mainnet-relay:3001", "1234596", "1"])
        .add_row(vec!["preview", "some-preview-relay:3001", "1234", "1"])
        .add_row(vec!["preprod", "some-preprod-relay:3001", "12345", "2"])
        .add_row(vec!["mainnet2", "some-mainnet-relay:3001", "0", "0"]);

    println!("{table}");

    Ok(())
}
