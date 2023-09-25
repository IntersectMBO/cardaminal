use clap::Parser;
use serde::Serialize;
use tracing::{info, instrument};

#[derive(Parser, Serialize)]
pub struct Args {
    /// Name of the new chain
    #[arg(short, long, env = "CARDAMINAL_CHAIN_NAME")]
    name: String,
    /// Chain N2N connection string
    #[arg(short, long, env = "CARDAMINAL_CHAIN_SOURCE")]
    source: String,
}

#[instrument("update", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!(
        "Chain created \n{}",
        serde_json::to_string_pretty(&args).unwrap()
    );
    Ok(())
}
