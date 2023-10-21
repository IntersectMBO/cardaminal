use std::fs;

use clap::Parser;
use miette::{bail, IntoDiagnostic};
use pallas::ledger::addresses::{
    Network, ShelleyAddress, ShelleyDelegationPart, ShelleyPaymentPart,
};
use tracing::{info, instrument};

use crate::{
    chain::config::Chain,
    wallet::{config::Wallet, dal::WalletDB, keys},
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

    let password = match &args.password {
        Some(p) => p,
        None => bail!("password is required"),
    };

    let wallet_slug = slug::slugify(&args.name);

    let wallet_path = ctx.dirs.root_dir.join("wallets").join(&wallet_slug);
    if wallet_path.exists() {
        bail!("wallet already exists")
    }

    fs::create_dir_all(&wallet_path).into_diagnostic()?;

    // open wallet db
    let db = WalletDB::open(&args.name, &wallet_path)
        .await
        .into_diagnostic()?;

    // create required tables in db
    db.migrate_up().await.into_diagnostic()?;

    // TODO: generate keys using pallas
    let (priv_key, pkh) = keys::temp_keygen();

    let encrypted_priv_key = keys::encrypt_privkey(password, priv_key);

    fs::write(wallet_path.join("privkey.enc"), encrypted_priv_key.clone()).into_diagnostic()?;
    fs::write(wallet_path.join("pkh.pub"), pkh).into_diagnostic()?;

    let mainnet_address = ShelleyAddress::new(
        Network::Mainnet,
        ShelleyPaymentPart::key_hash(pkh.into()),
        ShelleyDelegationPart::Null,
    );

    fs::write(
        wallet_path.join("address_mainnet_enterprise"),
        mainnet_address.to_bech32().unwrap(),
    )
    .into_diagnostic()?;

    let testnet_address = ShelleyAddress::new(
        Network::Testnet,
        ShelleyPaymentPart::key_hash(pkh.into()),
        ShelleyDelegationPart::Null,
    );

    fs::write(
        wallet_path.join("address_testnet_enterprise"),
        testnet_address.to_bech32().unwrap(),
    )
    .into_diagnostic()?;

    let wallet: Wallet = (&args).into();

    wallet.save_config(&ctx.dirs.root_dir)?;

    info!(wallet = args.name, "created");
    Ok(())
}
