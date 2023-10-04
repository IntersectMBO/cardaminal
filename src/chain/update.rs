use clap::Parser;
use indicatif::ProgressStyle;
use miette::{bail, IntoDiagnostic};
use pallas::{
    ledger::traverse::MultiEraHeader,
    network::miniprotocols::{chainsync::NextResponse, Point},
};
use tracing::{info, info_span, instrument};
use tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::chain::{
    config::Chain,
    sources::{n2n::bootstrap, IntersectConfig},
};

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to update
    name: String,
}

#[instrument("update", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    info!(chain = args.name, "updating");

    let chain_path = ctx.dirs.root_dir.join("chains").join(&args.name);
    let chain = Chain::from_path(&chain_path)?;
    if chain.is_none() {
        bail!("chain not exist")
    }

    let chain = chain.unwrap();

    //TODO: load latest point sync from chain database or get from after config

    let mut peer_client = bootstrap(&chain, IntersectConfig::Origin).await?;

    let chainsync = peer_client.chainsync();

    let (_, tip) = chainsync
        .find_intersect(vec![Point::Origin])
        .await
        .into_diagnostic()?;

    let mut slot: u64 = 0;
    let slot_tip = tip.0.slot_or_default();

    let span = info_span!("chain-update");
    span.pb_set_style(&ProgressStyle::default_bar());
    span.pb_set_length(slot_tip);

    span.pb_set_style(
        &ProgressStyle::with_template(
            "{spinner:.white} [{elapsed_precise}] [{wide_bar:.white/white}] {pos}/{len}",
        )
        .unwrap(),
    );

    let span_enter = span.enter();

    while slot < slot_tip {
        let response = chainsync.request_next().await.into_diagnostic()?;

        match response {
            NextResponse::RollForward(header, _) => {
                let header = match header.byron_prefix {
                    Some((subtag, _)) => {
                        MultiEraHeader::decode(header.variant, Some(subtag), &header.cbor)
                    }
                    None => MultiEraHeader::decode(header.variant, None, &header.cbor),
                }
                .into_diagnostic()?;

                slot = header.slot();

                //TODO: open content and save in db

                span.pb_set_position(slot);
                info!(last_slot = slot, "new blocks downloaded");
            }
            NextResponse::RollBackward(point, _) => {
                //TODO: validate rollback

                slot = point.slot_or_default();
                span.pb_set_position(slot);
            }
            NextResponse::Await => {
                break;
            }
        };
    }

    std::mem::drop(span_enter);
    std::mem::drop(span);

    info!("chain updated");

    Ok(())
}
