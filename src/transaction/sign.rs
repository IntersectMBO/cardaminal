use clap::Parser;
use miette::{bail, IntoDiagnostic};
use pallas::{
    crypto::key::ed25519,
    ledger::primitives::{conway::VKeyWitness, Fragment},
    txbuilder::transaction::Transaction,
};
use tracing::{info, instrument};

use crate::{
    transaction::model::{
        built::{BuiltTransaction, Bytes64},
        Bytes,
    },
    wallet::{
        config::Wallet,
        dal::{entities::transaction::Status, WalletDB},
        keys::decrypt_privkey,
    },
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
    /// transaction id
    id: i32,
    /// wallet name
    wallet: String,

    /// wallet password for signature
    #[arg(long, short, action)]
    password: Option<String>,
    /// use interactive mode
    #[arg(long, short, action)]
    interactive: bool,
}

#[instrument("build", skip_all, fields())]
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

    let privkey = decrypt_privkey(
        password,
        hex::decode(wallet.keys.private_encrypted)
            .map_err(|_| miette::miette!("malformed encrypted private key"))?,
    )
    .map_err(|_| miette::miette!("could not decrypt private key"))?;

    let privkey = ed25519::SecretKey::from(privkey);

    let pubkey: [u8; 64] = privkey
        .public_key()
        .as_ref()
        .try_into()
        .map_err(|_| miette::miette!("malformed public key"))?;

    let signature: [u8; 64] = built_tx
        .sign(privkey)
        .as_ref()
        .try_into()
        .map_err(|_| miette::miette!("malformed signature"))?;

    // add signature to json field

    let mut new_sigs = built_tx.signatures.unwrap_or_default();

    new_sigs.insert(Bytes64(pubkey), Bytes64(signature));

    built_tx.signatures = Some(new_sigs);

    // add signature to tx bytes

    let mut tx =
        Transaction::decode_fragment(&record.tx_cbor.ok_or(miette::miette!("tx cbor missing"))?)
            .map_err(|e| miette::miette!("malformed tx cbor: {e:?}"))?;

    let mut vkey_witnesses = tx.witness_set.vkeywitness.unwrap_or(vec![]);

    vkey_witnesses.push(VKeyWitness {
        vkey: Vec::from(pubkey.as_ref()).into(),
        signature: Vec::from(signature.as_ref()).into(),
    });

    tx.witness_set.vkeywitness = Some(vkey_witnesses);

    built_tx.tx_bytes = Bytes(tx.encode_fragment().unwrap());

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
