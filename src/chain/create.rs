use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// Name of the new chain
    #[arg(short, long, env = "CARDAMINAL_CHAIN_NAME")]
    name: String,
    /// Chain N2N connection string
    #[arg(short, long, env = "CARDAMINAL_CHAIN_SOURCE")]
    source: String,
}

#[instrument("update", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    info!("chain created");
    Ok(())
}
