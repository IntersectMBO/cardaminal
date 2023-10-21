use std::{fs, io::Write};

use clap::Parser;
use miette::{bail, IntoDiagnostic};
use tracing::{info, instrument};

use crate::chain::config::Chain;

#[derive(Parser)]
pub struct Args {
    /// friendly name to identify the chain
    pub name: String,

    /// [host]:[port] of the upstream node
    pub upstream: String,

    /// network magic of the chain
    pub magic: String,

    /// network id for addresses
    pub address_network_id: u8,

    /// [slot],[hash] of the sync start point
    #[arg(short, long)]
    pub after: Option<String>,
}

#[instrument("create", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let chain: Chain = (&args).try_into()?;

    let chain_slug = slug::slugify(&args.name);

    let chain_path = ctx.dirs.root_dir.join("chains").join(&chain_slug);
    if chain_path.exists() {
        bail!("chain already exists")
    }

    fs::create_dir_all(&chain_path).into_diagnostic()?;

    let toml_string = toml::to_string(&chain).into_diagnostic()?;
    let mut file = fs::File::create(chain_path.join("config.toml")).into_diagnostic()?;
    file.write_all(toml_string.as_bytes()).into_diagnostic()?;

    info!(chain = chain_slug, "chain created");

    Ok(())
}
