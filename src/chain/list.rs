use std::{
    fs,
    io::{BufReader, Read},
};

use clap::Parser;
use miette::{bail, IntoDiagnostic};
use tracing::instrument;

use crate::{
    chain::config::{Chain, ChainFormatter},
    OutputFormat,
};

#[derive(Parser)]
pub struct Args {}

#[instrument("list", skip_all)]
pub async fn run(_args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let chains_path = ctx.dirs.root_dir.join("chains");
    if !chains_path.exists() {
        bail!("no network registered")
    }

    let mut chains: Vec<Chain> = Vec::new();
    for dir in fs::read_dir(chains_path).into_diagnostic()? {
        let dir = dir.into_diagnostic()?;
        let config_path = dir.path().join("config.toml");
        if config_path.exists() {
            let file = fs::File::open(config_path).into_diagnostic()?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).into_diagnostic()?;

            let chain: Chain = toml::from_str(&contents).into_diagnostic()?;
            chains.push(chain);
        }
    }

    match ctx.output_format {
        OutputFormat::Json => chains.to_json(),
        OutputFormat::Table => chains.to_table(),
    }

    Ok(())
}
