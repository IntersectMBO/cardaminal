use clap::Parser;
use miette::bail;
use tracing::instrument;

use super::config;

#[derive(Parser)]
pub struct Args {
    /// name of the chain to delete
    name: String,

    /// Show testnet address instead of mainnet
    #[arg(long, short, action)]
    testnet: bool,
}

#[instrument("info", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let cfg = config::Wallet::load_config(&ctx.dirs.root_dir, &args.name)?;

    let cfg = match cfg {
        Some(x) => x,
        None => bail!("wallet doesn't exist"),
    };

    if args.testnet {
        println!("{}", cfg.addresses.testnet);
    } else {
        println!("{}", cfg.addresses.mainnet);
    }

    Ok(())
}
