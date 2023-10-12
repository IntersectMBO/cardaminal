use std::fs;

use clap::Parser;
use miette::{bail, IntoDiagnostic};
use tracing::{info, instrument};

use crate::{
    chain::config::Chain,
    wallet::{config::Wallet, dal::WalletDB},
};

pub fn gather_inputs(args: &mut Args) -> miette::Result<()> {
    let password = inquire::Password::new("password:")
        .with_help_message("the spending password of your wallet")
        .with_display_mode(inquire::PasswordDisplayMode::Masked)
        .prompt()
        .into_diagnostic()?;

    args.password = Some(password);

    Ok(())
}

#[derive(Parser, Clone)]
pub struct Args {
    /// name to identify the wallet
    pub name: String,

    /// name of the chain to attach the wallet
    #[arg(short, long, env = "CARDAMINAL_DEFAULT_CHAIN")]
    pub chain: Option<String>,

    /// spending password used to encrypt the private keys
    #[arg(short, long)]
    password: Option<String>,

    /// use interactive mode
    #[arg(long, short, action)]
    interactive: bool,
}

#[instrument("create", skip_all)]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let mut args = args;

    let wallet_slug = slug::slugify(&args.name);
    let wallet_path = Wallet::dir(&ctx.dirs.root_dir, &wallet_slug);
    if wallet_path.exists() {
        bail!("wallet already exists")
    }

    if args.chain.is_some()
        && !Chain::dir(&ctx.dirs.root_dir, args.chain.as_ref().unwrap()).exists()
    {
        bail!("chain doesn't exist")
    }

    if args.interactive {
        gather_inputs(&mut args)?;
    }

    if args.password.is_none() {
        bail!("password is required")
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

    // TODO: encrypt keys with password
    // TODO: generate keys using pallas and save

    let wallet: Wallet = (&args).into();

    wallet.save_config(&ctx.dirs.root_dir)?;

    info!(wallet = args.name, "created");
    Ok(())
}
