use clap::Parser;
use miette::Context;
use tracing::instrument;

use crate::{utils::OutputFormatter, OutputFormat};

use super::config::Wallet;

#[derive(Parser)]
pub struct Args {
    #[arg(long, default_value = "false")]
    ignore_errors: bool,
}

#[instrument("list", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallets: Vec<_> = Wallet::list_available(&ctx.dirs.root_dir)
        .context("error listing wallets")?
        .into_iter()
        .map(|name| {
            Wallet::load_config(&ctx.dirs.root_dir, &name).context(format!("loading wallet {name}"))
        })
        .filter(|cfg| cfg.is_ok() || !args.ignore_errors)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();

    match ctx.output_format {
        OutputFormat::Json => wallets.to_json(),
        OutputFormat::Table => wallets.to_table(),
    }

    Ok(())
}
