use clap::{Parser, Subcommand};
use tracing::instrument;

mod set;
mod clear;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// set the TTL of the transaction (slot at which the transaction can no longer be included in the chain)
    Set(set::Args),
    /// clear/remove the TTL of a transaction
    Clear(clear::Args),
}

#[instrument("ttl", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    match args.command {
        Commands::Set(args) => set::run(args).await,
        Commands::Clear(args) => clear::run(args).await,
    }
}
