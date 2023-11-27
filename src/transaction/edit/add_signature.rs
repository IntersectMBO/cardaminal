use clap::Parser;
use miette::{bail, miette, Context, IntoDiagnostic};
use pallas::txbuilder::BuiltTransaction;
use tracing::{info, instrument};

use crate::wallet::{
    config::Wallet,
    dal::{entities::transaction::Status, WalletDB},
};

#[derive(Parser)]
pub struct Args {
    /// hex public key (not hash of key)
    public_key: String,
    /// hex signature for a witness from the transaction
    signature: String,
}

#[instrument("add signature", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &super::EditContext<'_>) -> miette::Result<()> {
    let public_key: [u8; 32] = hex::decode(args.public_key)
        .into_diagnostic()
        .context("parsing public key hex")?
        .try_into()
        .map_err(|_| miette!("public key incorrect length"))?;

    let signature: [u8; 64] = hex::decode(args.signature)
        .into_diagnostic()
        .context("parsing signature hex")?
        .try_into()
        .map_err(|_| miette!("signature incorrect length"))?;

    let wallet = Wallet::load_config(&ctx.global_ctx.dirs.root_dir, &ctx.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(
        &wallet.name,
        &Wallet::dir(&ctx.global_ctx.dirs.root_dir, &wallet.name),
    )
    .await
    .into_diagnostic()?;

    let mut record = wallet_db
        .fetch_by_id(&(ctx.tx_id as i32))
        .await
        .into_diagnostic()?
        .ok_or(miette::miette!("transaction doesn't exist"))?;

    match record.status {
        Status::Staging => bail!("transaction must be built before modifying signatures"),
        _ => (),
    }

    let mut built_tx: BuiltTransaction =
        serde_json::from_slice(&record.tx_json).into_diagnostic()?;

    built_tx = built_tx
        .add_signature(public_key.into(), signature)
        .into_diagnostic()?;

    // update db

    record.status = Status::Signed;
    record.tx_json = serde_json::to_vec(&built_tx).into_diagnostic()?;
    record.tx_cbor = Some(built_tx.tx_bytes.0);

    wallet_db
        .update_transaction(record)
        .await
        .into_diagnostic()?;

    info!("signature added to transaction");

    Ok(())
}
