use clap::Parser;
use miette::IntoDiagnostic;
use tracing::instrument;

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
    /// the file with the pending tx
    file: String,
    /// use interactive mode
    #[arg(long, short, action)]
    interactive: bool,
}

#[instrument("sign", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    gather_inputs()?;
    Ok(())
}
