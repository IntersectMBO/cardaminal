use clap::{Parser, Subcommand};
use tracing::instrument;

mod attach;
mod create;
mod detach;
mod history;
mod list;
mod update;
mod utxos;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create a new wallet
    Create(create::Args),
    /// list available wallets
    List(list::Args),
    /// update wallet state from chain
    Update(update::Args),
    /// attach existing wallet to chain
    Attach(attach::Args),
    /// detach existing wallet from chain
    Detach(detach::Args),
    /// show wallet history
    History(history::Args),
    /// list current utxos of a wallet
    Utxos(utxos::Args),
}

#[instrument("wallet", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Create(args) => create::run(args).await,
        Commands::List(args) => list::run(args).await,
        Commands::Update(args) => {
            crate::with_tracing();
            update::run(args).await
        }
        Commands::Attach(args) => {
            crate::with_tracing();
            attach::run(args).await
        }
        Commands::Detach(args) => {
            crate::with_tracing();
            detach::run(args).await
        }
        Commands::History(args) => history::run(args).await,
        Commands::Utxos(args) => utxos::run(args).await,
    }
}
