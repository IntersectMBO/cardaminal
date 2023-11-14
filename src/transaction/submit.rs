use std::{str::FromStr, time::Duration};

use clap::Parser;
use miette::{bail, Context, IntoDiagnostic};
use reqwest::header;
use tracing::{info, instrument};

use crate::wallet::{
    config::{SubmitApi, Wallet},
    dal::WalletDB,
};

#[derive(Parser)]
pub struct Args {
    /// name of the wallet
    #[arg(env = "CARDAMINAL_DEFAULT_WALLET")]
    wallet: String,
    /// transaction id
    id: i32,
}

#[instrument("submit", skip_all, fields(args))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let wallet = Wallet::load_config(&ctx.dirs.root_dir, &args.wallet)?
        .ok_or(miette::miette!("wallet doesn't exist"))?;

    let wallet_db = WalletDB::open(&wallet.name, &Wallet::dir(&ctx.dirs.root_dir, &wallet.name))
        .await
        .into_diagnostic()?;

    let transaction = wallet_db
        .fetch_by_id(&args.id)
        .await
        .into_diagnostic()?
        .ok_or(miette::miette!("transaction doesn't exist"))?;

    if transaction.tx_cbor.is_none() {
        bail!("transaction is not ready to submit yet")
    }

    let api = wallet.submit_api.unwrap_or_default();

    let headers = get_headers(&api)?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .default_headers(headers)
        .build()
        .unwrap();

    let response = client
        .post(api.url)
        .body(transaction.tx_cbor.unwrap())
        .send()
        .await
        .into_diagnostic()
        .context("fail to submit transaction")?;

    let status = response.status();
    if !status.is_success() {
        let message = response.text().await.into_diagnostic()?;

        bail!(format!(
            "fail to submit transaction. Error code: {}. Message: {}",
            status.as_u16(),
            message,
        ))
    }

    info!("transaction submitted");

    Ok(())
}

fn get_headers(api: &SubmitApi) -> miette::Result<header::HeaderMap> {
    let mut headers = header::HeaderMap::new();

    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/cbor").unwrap(),
    );

    for (key, value) in api.headers.iter() {
        headers.insert(
            header::HeaderName::from_str(key).unwrap(),
            header::HeaderValue::from_str(value).unwrap(),
        );
    }

    Ok(headers)
}
