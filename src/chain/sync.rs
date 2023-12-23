use clap::Parser;
use indicatif::ProgressStyle;
use pallas::network::miniprotocols::chainsync::Tip;
use tracing::{info, info_span, instrument, warn, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::chain::{config::Chain, upstream::Upstream};

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to synchronize
    name: String,
}

fn update_progress(span: &Span, start: u64, slot: Option<u64>, tip: &Option<Tip>) {
    if let Some(slot) = slot {
        span.pb_set_position(slot - start);
    }

    if let Some(tip) = tip {
        span.pb_set_length(tip.0.slot_or_default() - start);
    }
}

#[instrument("sync", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    info!(chain = args.name, "updating");

    let chain = Chain::load_config(&ctx.dirs.root_dir, &args.name)?
        .ok_or(miette::miette!("chain doesn't exist"))?;

    let db = Chain::load_db(&ctx.dirs.root_dir, &args.name)?;

    let mut upstream = Upstream::bootstrap(chain, db).await?;

    let span = info_span!("chain-sync");

    span.pb_set_style(
        &ProgressStyle::with_template(
            "{spinner:.white} [{elapsed_precise}] [{bar:.white/white}] {pos}/{len} [ETA: {eta}]",
        )
        .unwrap(),
    );

    let span = span.entered();

    loop {
        upstream.next_step(|_| {}).await?;

        update_progress(
            &span,
            upstream.start_slot,
            upstream.current_slot,
            &upstream.tip,
        );

        if upstream.is_tip {
            warn!("reached tip of the chain");
            break;
        }
    }

    info!("chain synchronized");

    Ok(())
}
