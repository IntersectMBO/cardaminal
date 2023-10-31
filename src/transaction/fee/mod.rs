use clap::{Parser, Subcommand};
use tracing::instrument;

mod clear;
mod set;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// manually set the transaction fee of a transaction. If no fee set Cardaminal will attempt to compute the fee
    Set(set::Args),
    /// clear/remove the transaction fee of a transaction. If no fee set Cardaminal will attempt to compute the fee
    Clear(clear::Args),
}

#[instrument("fee", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Set(args) => set::run(args).await,
        Commands::Clear(args) => clear::run(args).await,
    }
}
