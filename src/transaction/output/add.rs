use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// transaction id
    tx_id: String,
    /// output address
    address: String,
    /// output assets [policy][name]:[amount]
    assets: Vec<String>,

    /// datum hash
    #[arg(long, action)]
    datum: Option<String>,
    /// datum file path
    #[arg(long, action)]
    datum_file: Option<String>,

    /// reference script hash
    #[arg(long, action)]
    reference_script: Option<String>,
    /// reference script file path
    #[arg(long, action)]
    reference_script_file: Option<String>,
}

#[instrument("add", skip_all, fields())]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
