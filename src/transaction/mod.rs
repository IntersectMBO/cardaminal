use clap::{Parser, Subcommand};
use tracing::instrument;

mod build;
mod config;
mod create;
mod delete;
mod edit;
mod export;
mod inspect;
mod list;
mod model;
mod sign;
mod submit;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create a new empty transaction in the transaction staging area for the
    /// specified chain
    Create(create::Args),
    /// list transactions which are in the staging area, along with some
    /// information summary regarding the transaction
    List(list::Args),
    /// edit a transaction while still in the staging area
    Edit(edit::Args),
    /// remove a transaction from the transaction staging area
    Delete(delete::Args),
    /// detailed information on a specific transaction in the staging area
    Inspect(inspect::Args),
    /// build/finalize a transaction in the staging area so that it is ready for
    /// signatures to be attached
    Build(build::Args),
    /// sign a transaction using a Cardaminal wallet
    Sign(sign::Args),
    /// submit a transaction to cardano node
    Submit(submit::Args),
    /// export a transaction to json file
    Export(export::Args),
}

#[instrument("transaction", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    match args.command {
        Commands::Create(args) => create::run(args, ctx).await,
        Commands::List(args) => list::run(args, ctx).await,
        Commands::Edit(args) => edit::run(args, ctx).await,
        Commands::Delete(args) => delete::run(args, ctx).await,
        Commands::Inspect(args) => inspect::run(args, ctx).await,
        Commands::Build(args) => build::run(args).await,
        Commands::Sign(args) => sign::run(args).await,
        Commands::Submit(args) => submit::run(args, ctx).await,
        Commands::Export(args) => export::run(args, ctx).await,
    }
}
