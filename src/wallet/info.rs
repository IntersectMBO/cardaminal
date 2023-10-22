use clap::Parser;
use miette::bail;
use tracing::instrument;

use crate::{utils::OutputFormatter, OutputFormat};

use super::config;

#[derive(Parser)]
pub struct Args {
    /// name of the chain to delete
    name: String,
}

#[instrument("info", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let cfg = config::Wallet::load_config(&ctx.dirs.root_dir, &args.name)?;

    if cfg.is_none() {
        bail!("wallet doesn't exist");
    }

    let dir = config::Wallet::dir(&ctx.dirs.root_dir, &args.name);
    println!("local storage dir: {}", &dir.to_string_lossy());

    if let Some(cfg) = cfg {
        match ctx.output_format {
            OutputFormat::Table => cfg.to_table(),
            OutputFormat::Json => cfg.to_json(),
        }
    }

    Ok(())
}
