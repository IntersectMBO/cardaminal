use clap::{Parser, Subcommand};
use tracing::instrument;

mod add;
mod remove;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// manually add an already created signature to a built transaction
    Add(add::Args),
    /// remove a signature from a built transaction.
    Remove(remove::Args),
}

#[instrument("signatures", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Add(args) => add::run(args).await,
        Commands::Remove(args) => remove::run(args).await,
    }
}
