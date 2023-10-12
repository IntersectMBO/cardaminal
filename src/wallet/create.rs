use std::fs;

use clap::Parser;
use miette::{bail, IntoDiagnostic};
use tracing::{info, instrument};

use crate::wallet::dal::WalletDB;

pub fn gather_inputs() -> miette::Result<()> {
    let _ = inquire::Password::new("password:")
        .with_help_message("the spending password of your wallet")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .prompt()
        .into_diagnostic()?;

    Ok(())
}

#[derive(Parser)]
pub struct Args {
    /// name to identify the wallet
    name: String,

    /// name of the chain to attach the wallet
    #[arg(short, long, env = "CARDAMINAL_DEFAULT_CHAIN")]
    chain: Option<String>,

    /// spending password used to encrypt the private keys
    #[arg(short, long)]
    password: Option<String>,

    /// use interactive mode
    #[arg(long, short, action)]
    interactive: bool,
}

#[instrument("create", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    if args.interactive {
        gather_inputs()?;
    }

    let wallet_slug = slug::slugify(&args.name);

    let wallet_path = ctx.dirs.root_dir.join("wallets").join(&wallet_slug);
    if wallet_path.exists() {
        bail!("wallet already exists")
    }

    fs::create_dir_all(&wallet_path).into_diagnostic()?;

    // open wallet db
    let db = WalletDB::open(&args.name, wallet_path)
        .await
        .into_diagnostic()?;

    // create required tables in db
    db.migrate_up().await.into_diagnostic()?;

    info!(wallet = args.name, "created");
    Ok(())
}
