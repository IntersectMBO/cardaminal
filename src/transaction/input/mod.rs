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
    /// add an input to a transaction
    Add(add::Args),
    /// remove an input from a transaction
    Remove(remove::Args),
}

#[instrument("input", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    match args.command {
        Commands::Add(args) => add::run(args, ctx).await,
        Commands::Remove(args) => remove::run(args).await,
    }
}
