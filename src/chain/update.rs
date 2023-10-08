use clap::Parser;
use indicatif::ProgressStyle;
use miette::{bail, Context, IntoDiagnostic};
use pallas::{
    ledger::traverse::MultiEraHeader,
    network::{
        facades::PeerClient,
        miniprotocols::{
            chainsync::{NextResponse, Tip},
            Point,
        },
    },
};
use tracing::{info, info_span, instrument, warn, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::chain::config::Chain;

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to update
    name: String,
}

fn update_progress(span: &Span, slot: u64, tip: &Tip) {
    span.pb_set_position(slot);
    span.pb_set_length(tip.0.slot_or_default());
}

#[instrument("update", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    info!(chain = args.name, "updating");

    let chain_path = ctx.dirs.root_dir.join("chains").join(&args.name);
    let chain = Chain::from_path(&chain_path)?;
    if chain.is_none() {
        bail!("chain not exist")
    }

    let db_path = chain_path.join("db");
    let mut db = pallas::storage::rolldb::chain::Chain::open(&db_path)
        .into_diagnostic()
        .context("can't open chain db")?;

    let chain = chain.unwrap();

    let points: Vec<_> = db
        .intersect_options(5)
        .into_diagnostic()?
        .iter()
        .map(|(s, h)| Point::Specific(*s, h.to_vec()))
        .collect();

    info!(?points, "intersecting chain");

    let magic: u64 = chain.magic.parse().into_diagnostic()?;

    let mut peer_client = PeerClient::connect(&chain.upstream.address, magic)
        .await
        .into_diagnostic()?;

    if points.is_empty() {
        peer_client
            .chainsync()
            .intersect_origin()
            .await
            .into_diagnostic()?;
    } else {
        peer_client
            .chainsync()
            .find_intersect(points)
            .await
            .into_diagnostic()?;
    }

    let span = info_span!("chain-update");
    span.pb_set_style(&ProgressStyle::default_bar());
    span.pb_set_style(
        &ProgressStyle::with_template(
            "{spinner:.white} [{elapsed_precise}] [{wide_bar:.white/white}] {pos}/{len}",
        )
        .unwrap(),
    );

    let span = span.entered();

    loop {
        let response = peer_client
            .chainsync()
            .request_next()
            .await
            .into_diagnostic()?;

        match response {
            NextResponse::RollForward(header, tip) => {
                let header = match header.byron_prefix {
                    Some((subtag, _)) => {
                        MultiEraHeader::decode(header.variant, Some(subtag), &header.cbor)
                    }
                    None => MultiEraHeader::decode(header.variant, None, &header.cbor),
                }
                .into_diagnostic()?;

                let slot = header.slot();
                let hash = header.hash();

                let block = peer_client
                    .blockfetch()
                    .fetch_single(Point::Specific(slot, hash.to_vec()))
                    .await
                    .into_diagnostic()
                    .context("error fetching block from upstream peer")?;

                db.roll_forward(header.slot(), header.hash(), block)
                    .into_diagnostic()
                    .context("error saving block to db")?;

                info!(last_slot = slot, "new blocks downloaded");

                update_progress(&span, slot, &tip);
            }
            NextResponse::RollBackward(point, tip) => {
                match point {
                    Point::Origin => {
                        db.roll_back_origin()
                            .into_diagnostic()
                            .context("error saving block to db")?;

                        info!("rolled back to origin");

                        update_progress(&span, 0, &tip);
                    }
                    Point::Specific(slot, _hash) => {
                        //let hash = Hash::<32>::from(&hash[0..8]);
                        db.roll_back(slot)
                            .into_diagnostic()
                            .context("error saving block to db")?;

                        warn!(slot, "rolled back to slot");

                        update_progress(&span, slot, &tip);
                    }
                }
            }
            NextResponse::Await => {
                warn!("reached tip of the chain");
                break;
            }
        };
    }

    info!("chain updated");

    Ok(())
}
