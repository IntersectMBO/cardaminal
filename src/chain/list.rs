use clap::Parser;
use miette::Context;
use tracing::instrument;

use crate::{chain::config::Chain, utils::OutputFormatter, OutputFormat};

#[derive(Parser)]
pub struct Args {
    #[arg(long, default_value = "false")]
    ignore_errors: bool,
}

#[instrument("list", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let chains: Vec<_> = Chain::list_available(&ctx.dirs.root_dir)
        .context("error listing chains")?
        .into_iter()
        .map(|name| {
            Chain::load_config(&ctx.dirs.root_dir, &name).context(format!("loading chain {name}"))
        })
        .filter(|cfg| cfg.is_ok() || !args.ignore_errors)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    match ctx.output_format {
        OutputFormat::Json => chains.to_json(),
        OutputFormat::Table => chains.to_table(),
    }

    Ok(())
}
