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
    /// add an asset and amount to the transaction mint set, negative value for burn
    Add(add::Args),
    /// remove an asset from the transaction mint set
    Remove(remove::Args),
}

#[instrument("mint", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Add(args) => add::run(args).await,
        Commands::Remove(args) => remove::run(args).await,
    }
}
