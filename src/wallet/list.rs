use clap::Parser;
use miette::Context;
use tracing::instrument;

use crate::{utils::OutputFormatter, OutputFormat};

use super::config::Wallet;

#[derive(Parser)]
pub struct Args {}

#[instrument("list", skip_all)]
pub async fn run(_args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallets: Vec<Wallet> = Wallet::list_available(&ctx.dirs.root_dir)
        .context("error listing wallets")?
        .into_iter()
        .map(|name| Wallet::load_config(&ctx.dirs.root_dir, &name))
        .filter_map(|cfg| cfg.ok().flatten())
        .collect();

    match ctx.output_format {
        OutputFormat::Json => wallets.to_json(),
        OutputFormat::Table => wallets.to_table(),
    }

    Ok(())
}
