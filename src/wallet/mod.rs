use clap::{Parser, Subcommand};
use tracing::instrument;

mod attach;
mod balance;
mod config;
mod create;
mod dal;
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
    // show wallet balance
    Balance(balance::Args),
}

#[instrument("wallet", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    match args.command {
        Commands::Create(args) => create::run(args, ctx).await,
        Commands::List(args) => list::run(args, ctx).await,
        Commands::Update(args) => {
            crate::with_tracing();
            update::run(args, ctx).await
        }
        Commands::Attach(args) => {
            crate::with_tracing();
            attach::run(args, ctx).await
        }
        Commands::Detach(args) => detach::run(args, ctx).await,
        Commands::History(args) => history::run(args).await,
        Commands::Utxos(args) => utxos::run(args, ctx).await,
        Commands::Balance(args) => balance::run(args, ctx).await,
    }
}
