use clap::Parser;
use miette::IntoDiagnostic;
use tracing::{info, instrument};

pub fn gather_inputs() -> miette::Result<()> {
    let _ = inquire::Password::new("password:")
        .with_help_message("the spending password of your wallet")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .prompt()
        .into_diagnostic()?;

    Ok(())
}

#[derive(Parser)]
pub struct Args {
    /// name to identify the wallet
    name: String,

    /// name of the chain to attach the wallet
    #[arg(short, long, env = "CARDAMINAL_DEFAULT_CHAIN")]
    chain: Option<String>,

    /// spending password used to encrypt the private keys
    #[arg(short, long)]
    password: Option<String>,

    /// use interactive mode
    #[arg(long, short, action)]
    interactive: bool,
}

#[instrument("create", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    if args.interactive {
        gather_inputs()?;
    }

    info!(wallet = args.name, "created");
    Ok(())
}
