use clap::{Parser, Subcommand};
use tracing::instrument;

mod build;
mod changeaddress;
mod collateralinput;
mod collateraloutput;
mod config;
mod create;
mod datum;
mod delete;
mod disclosedsigner;
mod fee;
mod input;
mod inspect;
mod list;
mod mint;
mod model;
mod network;
mod output;
mod overridesignersamount;
mod redeemer;
mod referenceinput;
mod script;
mod sign;
mod signatures;
mod submit;
mod ttl;

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// create a new empty transaction in the transaction staging area for the specified chain
    Create(create::Args),
    /// list transactions which are in the staging area, along with some information summary regarding the transaction
    List(list::Args),
    /// remove a transaction from the transaction staging area
    Delete(delete::Args),
    /// detailed information on a specific transaction in the staging area
    Inspect(inspect::Args),
    /// build/finalize a transaction in the staging area so that it is ready for signatures to be attached
    Build(build::Args),
    /// sign a transaction using a Cardaminal wallet
    Sign(sign::Args),
    /// submit a transaction to cardano node
    Submit(submit::Args),
    /// manage inputs
    Input(input::Args),
    /// manage reference inputs
    ReferenceInput(referenceinput::Args),
    /// manage outputs
    Output(output::Args),
    /// manage fee
    Fee(fee::Args),
    /// manage mint assets
    Mint(mint::Args),
    /// manage ttl
    TTL(ttl::Args),
    /// manage network
    Network(network::Args),
    /// manage collateral input
    CollateralInput(collateralinput::Args),
    /// manage collateral output
    CollateralOutput(collateraloutput::Args),
    /// manage disclosed signers
    DisclosedSigner(disclosedsigner::Args),
    /// manage scripts
    Script(script::Args),
    /// manage datum
    Datum(datum::Args),
    /// manage redeemer
    Redeemer(redeemer::Args),
    /// manage override signers amount
    OverrideSignersAmount(overridesignersamount::Args),
    /// manage change address
    ChangeAddress(changeaddress::Args),
    /// manage signatures
    Signatures(signatures::Args),
}

#[instrument("transaction", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    match args.command {
        Commands::Create(args) => create::run(args, ctx).await,
        Commands::List(args) => list::run(args, ctx).await,
        Commands::Delete(args) => delete::run(args, ctx).await,
        Commands::Inspect(args) => inspect::run(args).await,
        Commands::Input(args) => input::run(args, ctx).await,
        Commands::ReferenceInput(args) => referenceinput::run(args).await,
        Commands::Output(args) => output::run(args).await,
        Commands::Fee(args) => fee::run(args).await,
        Commands::Mint(args) => mint::run(args).await,
        Commands::TTL(args) => ttl::run(args).await,
        Commands::Network(args) => network::run(args).await,
        Commands::CollateralInput(args) => collateralinput::run(args).await,
        Commands::CollateralOutput(args) => collateraloutput::run(args).await,
        Commands::DisclosedSigner(args) => disclosedsigner::run(args).await,
        Commands::Script(args) => script::run(args).await,
        Commands::Datum(args) => datum::run(args).await,
        Commands::Redeemer(args) => redeemer::run(args).await,
        Commands::OverrideSignersAmount(args) => overridesignersamount::run(args).await,
        Commands::ChangeAddress(args) => changeaddress::run(args).await,
        Commands::Build(args) => build::run(args).await,
        Commands::Sign(args) => sign::run(args).await,
        Commands::Signatures(args) => signatures::run(args).await,
        Commands::Submit(args) => submit::run(args).await,
    }
}
