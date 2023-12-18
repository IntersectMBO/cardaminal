use clap::Parser;
use miette::{bail, IntoDiagnostic};
use pallas::{txbuilder::BuiltTransaction, wallet::wrapper};
use tracing::{info, instrument};

use crate::wallet::{
    config::Wallet,
    dal::{entities::transaction::Status, WalletDB},
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

#[derive(Parser)]
pub struct Args {
    /// wallet name
    wallet: String,
    /// transaction id
    id: i32,

    /// wallet password for signature
    #[arg(long, short, action)]
    password: Option<String>,
    /// use interactive mode
    #[arg(long, short, action)]
    interactive: bool,
}

#[instrument("sign", skip_all, fields())]
pub async fn run(mut args: Args, ctx: &crate::Context) -> miette::Result<()> {
    if args.interactive {
        gather_inputs(&mut args)?;
    }

    let password = match &args.password {
        Some(p) => p,
        None => bail!("password is required"),
    };

    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let mut record = wallet_db
        .fetch_by_id(&args.id)
        .await
        .into_diagnostic()?
        .ok_or(miette::miette!("transaction doesn't exist"))?;

    match record.status {
        Status::Staging => bail!("transaction must be built before signing"),
        _ => (),
    }

    let mut built_tx: BuiltTransaction =
        serde_json::from_slice(&record.tx_json).into_diagnostic()?;

    let privkey = wrapper::decrypt_private_key(
        password,
        hex::decode(wallet.keys.private_encrypted)
            .map_err(|_| miette::miette!("malformed encrypted private key"))?,
    )
    .map_err(|_| miette::miette!("could not decrypt private key"))?;

    built_tx = built_tx.sign(privkey).into_diagnostic()?;

    // update db

    record.status = Status::Signed;
    record.tx_json = serde_json::to_vec(&built_tx).into_diagnostic()?;
    record.tx_cbor = Some(built_tx.tx_bytes.0);

    wallet_db
        .update_transaction(record)
        .await
        .into_diagnostic()?;

    info!("transaction signed");

    Ok(())
}
