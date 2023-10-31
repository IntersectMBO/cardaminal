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
    /// add a public key hash to the required signers set of a transaction, so that it is disclosed to any Plutus scripts being executed by the transaction
    Add(add::Args),
    /// remove a public key hash from the required signers set of a transaction
    Remove(remove::Args),
}

#[instrument("disclosed-signer", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Add(args) => add::run(args).await,
        Commands::Remove(args) => remove::run(args).await,
    }
}
