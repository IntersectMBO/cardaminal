use clap::Parser;
use miette::Context;
use tracing::instrument;

use crate::{chain::config::Chain, utils::OutputFormatter, OutputFormat};

#[derive(Parser)]
pub struct Args {}

#[instrument("list", skip_all)]
pub async fn run(_args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let chains: Vec<Chain> = Chain::list_available(&ctx.dirs.root_dir)
        .context("error listing chains")?
        .into_iter()
        .map(|name| Chain::load_config(&ctx.dirs.root_dir, &name))
        .filter_map(|cfg| cfg.ok().flatten())
        .collect();

    match ctx.output_format {
        OutputFormat::Json => chains.to_json(),
        OutputFormat::Table => chains.to_table(),
    }

    Ok(())
}
