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
    /// for the purpose of fee computation Cardaminal will attempt to compute the minimum required number of signers the transaction has, but this can be overrided if more signatures will be attached to the transaction to ensure the fee computation is sufficient
    Set(set::Args),
    /// clear the overrided expected number of transaction signers for a transaction
    Clear(clear::Args),
}

#[instrument("override-signers-amount", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Set(args) => set::run(args).await,
        Commands::Clear(args) => clear::run(args).await,
    }
}
