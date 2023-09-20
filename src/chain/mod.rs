use clap::{Parser, Subcommand};
use tracing::instrument;

mod create;
mod list;
mod delete;
mod update;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new chain config
    Create(create::Args),
    /// List all chains configued
    List(list::Args),
    /// Delete a chain by name
    Delete(delete::Args),
    /// Sync a chain to latest point
    Update(update::Args),
}

#[instrument("chain", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Create(args) => create::run(args).await,
        Commands::List(args) => list::run(args).await,
        Commands::Delete(args) => delete::run(args).await,
        Commands::Update(args) => update::run(args).await,
    }
}
