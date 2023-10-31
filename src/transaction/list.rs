use clap::{Parser, ValueEnum};
use tracing::instrument;

#[derive(Clone, ValueEnum)]
enum TransactionStatus {
    Building,
    Signing,
    Submitted,
}

#[derive(Parser)]
pub struct Args {
    /// only return transactions with the specified status
    #[arg(long, short, action)]
    status: Option<TransactionStatus>,
}

#[instrument("list", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
