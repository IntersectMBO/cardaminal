use std::time::Duration;

use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// the file with the pending tx
    file: String,
}

#[instrument("submit", skip_all, fields(file=args.file))]
pub async fn run(args: Args) -> miette::Result<()> {
    // TODO: check all parameters available
    info!("submitting");

    tokio::time::sleep(Duration::from_secs(2)).await;

    info!("submitted");

    Ok(())
}
