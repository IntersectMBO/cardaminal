use clap::Parser;
use miette::IntoDiagnostic;
use tracing::instrument;

pub fn gather_inputs() -> miette::Result<()> {
    let _ = inquire::MultiSelect::new(
        "inputs:",
        vec![
            "abcabc#0 (24 ADA)",
            "abcabc#1 (3 ADA)",
            "abcabc#0 (8 ADA)",
            "abcabc#0 (24 ADA)",
            "abcabc#1 (3 ADA)",
        ],
    )
    .with_help_message("select the inputs UTxOs from your wallet")
    .prompt()
    .into_diagnostic()?;

    let _ = inquire::Text::new("output address:")
        .with_help_message("the address of your output")
        .prompt()
        .into_diagnostic()?;

    let _ = inquire::Text::new("output coin:")
        .with_help_message("the lovelace amount of your amount")
        .prompt()
        .into_diagnostic()?;

    let _ = inquire::Confirm::new("add another output?")
        .prompt()
        .into_diagnostic()?;

    Ok(())
}

#[derive(Parser)]
pub struct Args {
    /// the file to store the transaction
    file: String,
    /// use interactive mode
    #[arg(long, short, action)]
    interactive: bool,
}

#[instrument("build", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    gather_inputs()?;
    Ok(())
}
