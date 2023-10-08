use std::fs;

use clap::Parser;
use comfy_table::Table;
use miette::{bail, IntoDiagnostic};
use tracing::instrument;

use crate::{
    chain::config::{Chain, ChainFormatter},
    OutputFormat,
};

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to dump
    name: String,

    /// Number of blocks to skip
    #[arg(short, long, default_value_t = 0)]
    skip: usize,

    /// Number of blocks to output
    #[arg(short, long, default_value_t = 20)]
    take: usize,
}

#[instrument("dump", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let db = Chain::load_db(&ctx.dirs.root_dir, &args.name)?;

    let mut table = Table::new();
    table.set_header(vec!["slot", "hash"]);

    db.crawl()
        .skip(args.skip)
        .take(args.take)
        .filter_map(|r| r.ok())
        .for_each(|(s, h)| {
            table.add_row(vec![s.to_string(), h.to_string()]);
        });

    println!("{table}");

    Ok(())
}
