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
    /// set the network ID of a transaction
    Set(set::Args),
    /// clear/remove the network ID of a transaction
    Clear(clear::Args),
}

#[instrument("network", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Set(args) => set::run(args).await,
        Commands::Clear(args) => clear::run(args).await,
    }
}
