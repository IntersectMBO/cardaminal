use std::time::Duration;

use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// transaction id
    tx_id: String,
    /// chain name
    chain: String,
}

#[instrument("submit", skip_all, fields(tx_id=args.tx_id, chain=args.chain))]
pub async fn run(args: Args) -> miette::Result<()> {
    // TODO: check all parameters available
    info!("submitting");

    tokio::time::sleep(Duration::from_secs(2)).await;

    info!("submitted");

    Ok(())
}
