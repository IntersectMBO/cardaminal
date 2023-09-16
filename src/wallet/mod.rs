use clap::{Parser, Subcommand};
use tracing::instrument;

mod create;
mod update;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create(create::Args),
    Update(update::Args),
}

#[instrument("wallet", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Create(args) => create::run(args).await,
        Commands::Update(args) => update::run(args).await,
    }
}
