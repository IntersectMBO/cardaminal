use clap::Parser;
use tracing::instrument;

#[derive(Parser)]
pub struct Args {
    /// the file with the pending tx
    file: String,
    /// use interactive mode
    #[arg(long, short, action)]
    interactive: bool,
}

#[instrument("edit", skip_all)]
pub async fn run(_args: Args) -> miette::Result<()> {
    Ok(())
}
