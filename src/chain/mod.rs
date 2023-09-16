use clap::{Parser, Subcommand};
use tracing::instrument;

mod list;
mod update;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Update(update::Args),
    List(list::Args),
}

#[instrument("chain", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Update(args) => update::run(args).await,
        Commands::List(args) => list::run(args).await,
    }
}
