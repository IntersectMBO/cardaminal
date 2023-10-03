use std::fs;

use clap::Parser;
use miette::{bail, IntoDiagnostic};
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// name of the chain to delete
    name: String,

    /// automatically detach any wallets using this chain
    #[arg(long, default_value_t)]
    detach: bool,
}

#[instrument("delete", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let chain_path = ctx.dirs.root_dir.join("chains").join(&args.name);
    if !chain_path.exists() {
        bail!("chain not exist")
    }

    let confirm =
        inquire::Confirm::new(&format!("Do you confirm deleting the {} chain?", args.name))
            .prompt()
            .into_diagnostic()?;

    if confirm {
        fs::remove_dir_all(chain_path).into_diagnostic()?;

        // TODO: check how to detach param works

        info!("chain deleted");
    }

    Ok(())
}
