use clap::{Parser, ValueEnum};
use miette::bail;
use serde::Serialize;
use tracing::{info, instrument};

#[derive(ValueEnum, Clone, Serialize)]
pub enum Kind {
    N2N,
    N2C,
}

#[derive(Parser, Serialize)]
pub struct Args {
    /// Name of the new chain
    #[arg(short, long)]
    name: String,
    /// Chain connection type
    #[arg(short, long)]
    kind: Kind,
    /// Chain connection string
    #[arg(short, long)]
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
