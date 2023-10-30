use clap::{Parser, Subcommand, ValueEnum};
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
    /// add a redeemer to a transaction. If execution unit budget not specified Cardaminal will attempt to compute the required budget
    Add(add::Args),
    /// remove a redeemer from a transaction
    Remove(remove::Args),
}

#[instrument("redeemer", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Add(args) => add::run(args).await,
        Commands::Remove(args) => remove::run(args).await,
    }
}

#[derive(Clone, ValueEnum)]
pub enum RedeemerAction {
    Spend,
    Mint,
}
