use std::{fs, io::Write};

use clap::Parser;
use miette::IntoDiagnostic;
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

    /// [slot],[hash] of the sync start point
    #[arg(short, long)]
    pub after: Option<String>,
}

#[instrument("create", skip_all, fields(name=args.name))]
pub async fn run(args: Args) -> miette::Result<()> {
    let chain: Chain = (&args).try_into()?;

    let project_dir = directories::ProjectDirs::from("", "TxPipe", "cardaminal")
        .expect("Use root_dir parameter or env");

    // TODO: check how to get clap global parameter root_dir

    let root_dir = project_dir.data_dir();
    if !root_dir.exists() {
        fs::create_dir(root_dir.clone()).into_diagnostic()?;
    }

    let chains_dir = root_dir.join("chains");
    if !chains_dir.exists() {
        fs::create_dir(chains_dir.clone()).into_diagnostic()?;
    }

    let chain_slug = slug::slugify(&args.name);
    let chain_dir = chains_dir.join(&chain_slug);
    if chain_dir.exists() {
        return Err(miette::ErrReport::msg(format!(
            "the {} chain already exists",
            args.name
        )));
    }

    fs::create_dir(chain_dir.clone()).into_diagnostic()?;

    let toml_string = toml::to_string(&chain).into_diagnostic()?;
    let mut file = fs::File::create(chain_dir.join("config.toml")).into_diagnostic()?;
    file.write_all(toml_string.as_bytes()).into_diagnostic()?;

    info!(chain = chain_slug, "chain created");

    Ok(())
}
