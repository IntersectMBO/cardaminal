use clap::{Parser, Subcommand};
use tracing::instrument;

mod config;
mod create;
mod delete;
mod list;
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
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    match args.command {
        Commands::Create(args) => {
            crate::with_tracing();
            create::run(args, ctx).await
        }
        Commands::List(args) => list::run(args, ctx).await,
        Commands::Delete(args) => delete::run(args, ctx).await,
        Commands::Update(args) => {
            crate::with_tracing();
            update::run(args, ctx).await
        }
    }
}
