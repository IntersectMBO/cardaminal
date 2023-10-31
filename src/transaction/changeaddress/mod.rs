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
    /// set the change address for the transaction which will be used if Cardaminal is responsible for balancing the transaction and/or computing the transaction fee. An output will be created sending any unclaimed value to the change address
    Set(set::Args),
    /// clear the change address for a transaction
    Clear(clear::Args),
}

#[instrument("change-address", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Set(args) => set::run(args).await,
        Commands::Clear(args) => clear::run(args).await,
    }
}
