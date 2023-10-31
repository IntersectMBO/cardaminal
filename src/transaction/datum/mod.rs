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
    /// add a datum to a transaction (witness set)
    Add(add::Args),
    /// remove a datum from a transaction (witness set)
    Remove(remove::Args),
}

#[instrument("datum", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Add(args) => add::run(args).await,
        Commands::Remove(args) => remove::run(args).await,
    }
}
