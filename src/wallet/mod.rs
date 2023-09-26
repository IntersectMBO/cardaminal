use clap::{Parser, Subcommand};
use tracing::instrument;

mod attach;
mod create;
mod detach;
mod history;
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
    Attach(attach::Args),
    Detach(detach::Args),
    History(history::Args),
}

#[instrument("wallet", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Create(args) => create::run(args).await,
        Commands::Update(args) => update::run(args).await,
        Commands::Attach(args) => attach::run(args).await,
        Commands::Detach(args) => detach::run(args).await,
        Commands::History(args) => history::run(args).await,
    }
}
