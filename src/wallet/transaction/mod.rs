use clap::{Parser, Subcommand};
use tracing::instrument;

mod create;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create(create::Args),
}

#[instrument("transaction", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Create(args) => create::run(args).await,
    }
}
