use clap::{Parser, Subcommand, ValueEnum};
use tracing::instrument;

mod add_collateral_input;
mod add_datum;
mod add_disclosed_signer;
mod add_input;
mod add_mint;
mod add_output;
mod add_redeemer;
mod add_reference_input;
mod add_script;
mod add_signature;
mod clear_change_address;
mod clear_collateral_output;
mod clear_fee;
mod clear_network;
mod clear_signer_amount;
mod clear_ttl;
mod clear_valid_hereafter;
mod common;
mod remove_collateral_input;
mod remove_datum;
mod remove_disclosed_signer;
mod remove_input;
mod remove_mint;
mod remove_output;
mod remove_redeemer;
mod remove_reference_input;
mod remove_script;
mod remove_signature;
mod set_change_address;
mod set_collateral_output;
mod set_fee;
mod set_network;
mod set_signer_amount;
mod set_ttl;
mod set_valid_hereafter;

#[derive(Clone, ValueEnum)]
pub enum RedeemerAction {
    Spend,
    Mint,
}

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,

    /// transaction id
    id: u32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// add an input to a transaction
    AddInput(add_input::Args),
    /// remove an input from a transaction
    RemoveInput(remove_input::Args),
    /// adds a reference input to the transaction
    AddReferenceInput(add_reference_input::Args),
    /// removes a reference input from the transaction
    RemoveReferenceInput(remove_reference_input::Args),
    /// add an output to a transaction
    AddOutput(add_output::Args),
    /// remove an output from a transaction
    RemoveOutput(remove_output::Args),
    /// set the tx fee
    SetFee(set_fee::Args),
    /// clear the tx fee
    ClearFee(clear_fee::Args),
    /// adds a mint to the transaction
    AddMint(add_mint::Args),
    /// removes a mint from the transaction
    RemoveMint(remove_mint::Args),
    /// set the transaction ttl
    SetTtl(set_ttl::Args),
    /// clears the transaction ttl
    ClearTtl(clear_ttl::Args),
    /// set the transaction valid hereafter
    SetValidHereafter(set_valid_hereafter::Args),
    /// clear the transaction valid hereafter
    ClearValidHereafter(clear_valid_hereafter::Args),
    /// set the network for the transaction
    SetNetwork(set_network::Args),
    /// clear the transaction network
    ClearNetwork(clear_network::Args),
    /// adds a collateral input
    AddCollateralInput(add_collateral_input::Args),
    /// removes a collateral input
    RemoveCollateralInput(remove_collateral_input::Args),
    /// set collateral output
    SetCollateralOutput(set_collateral_output::Args),
    /// clear collateral output
    ClearCollateralOutput(clear_collateral_output::Args),
    /// adds a disclosed signer from the transaction
    AddDisclosedSigner(add_disclosed_signer::Args),
    /// removes a disclosed signer from the transaction
    RemoveDisclosedSigner(remove_disclosed_signer::Args),
    /// adds a script to the transaction
    AddScript(add_script::Args),
    /// removes a scripts from the transaction
    RemoveScript(remove_script::Args),
    /// adds a datum to a transaction
    AddDatum(add_datum::Args),
    /// removes a datum from a transaction
    RemoveDatum(remove_datum::Args),
    /// adds a redeemer to the transaction
    AddRedeemer(add_redeemer::Args),
    /// removes a redeemer from the transaction
    RemoveRedeemer(remove_redeemer::Args),
    /// override the amount of signers
    SetSignerAmount(set_signer_amount::Args),
    /// clear the amount of signers
    ClearSignerAmount(clear_signer_amount::Args),
    /// sets the change address
    SetChangeAddress(set_change_address::Args),
    /// clear the change address
    ClearChangeAddress(clear_change_address::Args),
    /// add signature to transaction
    AddSignature(add_signature::Args),
    /// remove signature from transaction
    RemoveSignature(remove_signature::Args),
}

pub struct EditContext<'a> {
    global_ctx: &'a crate::Context,
    tx_id: u32,
    wallet: String,
}

#[instrument("transaction", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let edit_ctx = EditContext {
        global_ctx: ctx,
        tx_id: args.id,
        wallet: args.wallet, //.ok_or(miette::miette!("no wallet specified"))?,
    };

    match args.command {
        Commands::AddInput(args) => add_input::run(args, &edit_ctx).await,
        Commands::RemoveInput(args) => remove_input::run(args, &edit_ctx).await,
        Commands::AddReferenceInput(args) => add_reference_input::run(args).await,
        Commands::RemoveReferenceInput(args) => remove_reference_input::run(args).await,
        Commands::AddOutput(args) => add_output::run(args, &edit_ctx).await,
        Commands::RemoveOutput(args) => remove_output::run(args, &edit_ctx).await,
        Commands::SetFee(args) => set_fee::run(args, &edit_ctx).await,
        Commands::ClearFee(args) => clear_fee::run(args, &edit_ctx).await,
        Commands::AddMint(args) => add_mint::run(args, &edit_ctx).await,
        Commands::RemoveMint(args) => remove_mint::run(args, &edit_ctx).await,
        Commands::SetTtl(args) => set_ttl::run(args).await,
        Commands::ClearTtl(args) => clear_ttl::run(args).await,
        Commands::SetValidHereafter(args) => set_valid_hereafter::run(args, &edit_ctx).await,
        Commands::ClearValidHereafter(args) => clear_valid_hereafter::run(args, &edit_ctx).await,
        Commands::SetNetwork(args) => set_network::run(args).await,
        Commands::ClearNetwork(args) => clear_network::run(args).await,
        Commands::AddCollateralInput(args) => add_collateral_input::run(args, &edit_ctx).await,
        Commands::RemoveCollateralInput(args) => remove_collateral_input::run(args, &edit_ctx).await,
        Commands::SetCollateralOutput(args) => set_collateral_output::run(args, &edit_ctx).await,
        Commands::ClearCollateralOutput(args) => {
            clear_collateral_output::run(args, &edit_ctx).await
        }
        Commands::AddDisclosedSigner(args) => add_disclosed_signer::run(args).await,
        Commands::RemoveDisclosedSigner(args) => remove_disclosed_signer::run(args).await,
        Commands::AddScript(args) => add_script::run(args, &edit_ctx).await,
        Commands::RemoveScript(args) => remove_script::run(args).await,
        Commands::AddDatum(args) => add_datum::run(args, &edit_ctx).await,
        Commands::RemoveDatum(args) => remove_datum::run(args, &edit_ctx).await,
        Commands::AddRedeemer(args) => add_redeemer::run(args).await,
        Commands::RemoveRedeemer(args) => remove_redeemer::run(args).await,
        Commands::SetSignerAmount(args) => set_signer_amount::run(args).await,
        Commands::ClearSignerAmount(args) => clear_signer_amount::run(args).await,
        Commands::SetChangeAddress(args) => set_change_address::run(args, &edit_ctx).await,
        Commands::ClearChangeAddress(args) => clear_change_address::run(args).await,
        Commands::AddSignature(args) => add_signature::run(args).await,
        Commands::RemoveSignature(args) => remove_signature::run(args).await,
    }
}
