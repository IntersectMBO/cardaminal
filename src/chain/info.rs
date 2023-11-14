use clap::Parser;
use miette::{bail, Context};
use tracing::instrument;

use super::config;

#[derive(Parser)]
pub struct Args {
    /// name of the chain to delete
    name: String,
}

#[instrument("info", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let cfg = config::Chain::load_config(&ctx.dirs.root_dir, &args.name)?;

    if cfg.is_none() {
        bail!("chain doesn't exist");
    }

    let dir = config::Chain::dir(&ctx.dirs.root_dir, &args.name);
    println!("local storage dir: {}", &dir.to_string_lossy());

    println!("{}", serde_json::to_string(&cfg).unwrap());

    Ok(())
}
