use clap::{Parser, Subcommand};
use tracing::instrument;

mod block;
pub mod config;
mod create;
mod delete;
mod dump;
mod info;
mod list;
mod sync;
mod upstream;
mod watch;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show info about an existing chain
    Info(info::Args),
    /// Create a new chain config
    Create(create::Args),
    /// List all chains configued
    List(list::Args),
    /// Delete a chain by name
    Delete(delete::Args),
    /// Sync a chain to latest point
    Sync(sync::Args),
    /// Watch the chain for specific data
    Watch(watch::Args),
    /// Dump data from the chain
    Dump(dump::Args),
    /// Show the content of a block
    Block(block::Args),
}

#[instrument("chain", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    match args.command {
        Commands::Info(args) => info::run(args, ctx).await,
        Commands::Create(args) => {
            crate::with_tracing();
            create::run(args, ctx).await
        }
        Commands::List(args) => list::run(args, ctx).await,
        Commands::Delete(args) => delete::run(args, ctx).await,
        Commands::Sync(args) => {
            crate::with_tracing();
            sync::run(args, ctx).await
        }
        Commands::Watch(args) => {
            crate::with_tracing();
            watch::run(args, ctx).await
        }
        Commands::Dump(args) => dump::run(args, ctx).await,
        Commands::Block(args) => block::run(args, ctx).await,
    }
}
