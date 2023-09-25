use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::Level;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::prelude::*;

pub mod chain;
pub mod transaction;
pub mod wallet;

pub struct Context {}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage chains
    Chain(chain::Args),
    /// Manage Wallets
    Wallet(wallet::Args),
    /// Manage Transactions
    Transaction(transaction::Args),
}

#[tokio::main]
async fn main() -> miette::Result<()> {
    let indicatif_layer = IndicatifLayer::new();

    tracing_subscriber::registry()
        //.with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(tracing_subscriber::filter::Targets::default().with_target("cardaminal", Level::INFO))
        .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
        .with(indicatif_layer)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Chain(args) => {
            // let ctx = Context::new(config, None, args.static_files)
            //     .into_diagnostic()
            //     .wrap_err("loading context failed")?;

            chain::run(args).await
        }
        Commands::Wallet(args) => {
            //let ctx = Context::load(cli.config, None, None).into_diagnostic()?;
            wallet::run(args).await
        }
        Commands::Transaction(args) => transaction::run(args).await,
    }
}
