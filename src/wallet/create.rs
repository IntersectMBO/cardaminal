use std::fs;

use clap::Parser;
use miette::{bail, IntoDiagnostic};
use pallas::ledger::addresses::{
    Network, ShelleyAddress, ShelleyDelegationPart, ShelleyPaymentPart,
};
use tracing::{info, instrument};

use crate::{chain, wallet};

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
    let wallet_path = wallet::config::Wallet::dir(&ctx.dirs.root_dir, &wallet_slug);
    if wallet_path.exists() {
        bail!("wallet already exists")
    }

    if args.chain.is_some()
        && !chain::config::Chain::dir(&ctx.dirs.root_dir, args.chain.as_ref().unwrap()).exists()
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
    let db = wallet::dal::WalletDB::open(&args.name, &wallet_path)
        .await
        .into_diagnostic()?;

    // create required tables in db
    db.migrate_up().await.into_diagnostic()?;

    let (priv_key, pkh) = wallet::keys::keygen();

    let encrypted_priv_key = wallet::keys::encrypt_privkey(password, priv_key);

    let key_data = wallet::config::Keys {
        public_key_hash: hex::encode(pkh),
        private_encrypted: hex::encode(encrypted_priv_key),
    };

    let mainnet_address = ShelleyAddress::new(
        Network::Mainnet,
        ShelleyPaymentPart::key_hash(pkh.into()),
        ShelleyDelegationPart::Null,
    );

    let testnet_address = ShelleyAddress::new(
        Network::Testnet,
        ShelleyPaymentPart::key_hash(pkh.into()),
        ShelleyDelegationPart::Null,
    );

    let addresses = wallet::config::Addresses {
        mainnet: mainnet_address.to_bech32().into_diagnostic()?,
        testnet: testnet_address.to_bech32().into_diagnostic()?,
    };

    let wallet = wallet::config::Wallet::new(args.name, key_data, addresses, args.chain);

    wallet.save_config(&ctx.dirs.root_dir)?;

    info!(wallet = wallet.name, "created");

    Ok(())
}
