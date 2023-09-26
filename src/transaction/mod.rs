use clap::{Parser, Subcommand};
use tracing::instrument;

mod build;
mod edit;
mod sign;
mod submit;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// build a new transaction
    Build(build::Args),
    /// edit existing transaction
    Edit(edit::Args),
    /// sign pending transaction
    Sign(sign::Args),
    /// submit pending transaction
    Submit(submit::Args),
}

#[instrument("transaction", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Build(args) => build::run(args).await,
        Commands::Edit(args) => edit::run(args).await,
        Commands::Sign(args) => sign::run(args).await,
        Commands::Submit(args) => {
            crate::with_tracing();
            submit::run(args).await
        }
    }
}
