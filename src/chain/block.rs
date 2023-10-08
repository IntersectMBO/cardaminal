use clap::Parser;
use comfy_table::Table;
use miette::{Context, IntoDiagnostic};
use pallas::crypto::hash::Hash;
use tracing::instrument;

use crate::chain::config::Chain;

#[derive(Parser)]
pub struct Args {
    /// Name of the chain that owns the block
    chain: String,

    /// Hash of the block to query
    hash: String,
}

#[instrument("block", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let db = Chain::load_db(&ctx.dirs.root_dir, &args.chain)?;

    let hash = hex::decode(args.hash)
        .into_diagnostic()
        .context("parsing hash hex")?;

    let hash = Hash::<32>::from(&hash[0..32]);

    let block = db
        .get_block(hash)
        .into_diagnostic()
        .context("fetching block from db")?
        .ok_or(miette::miette!("block not found"))?;

    let block = hex::encode(block);

    println!("{}", block);

    Ok(())
}
