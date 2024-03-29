use std::sync::atomic::AtomicBool;

use clap::Parser;
use indicatif::ProgressStyle;
use pallas::network::miniprotocols::chainsync::Tip;
use tracing::{info, info_span, instrument, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::chain::{config::Chain, upstream::Upstream};

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to watch
    name: String,

    /// Watch until tx hash is found
    #[arg(long)]
    tx_hash: Option<String>,
}

fn update_progress(span: &Span, start: u64, slot: Option<u64>, tip: &Option<Tip>) {
    if let Some(slot) = slot {
        span.pb_set_position(slot - start);
    }

    if let Some(tip) = tip {
        span.pb_set_length(tip.0.slot_or_default() - start);
    }
}

#[instrument("watch", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    info!(chain = args.name, "watching");

    let chain = Chain::load_config(&ctx.dirs.root_dir, &args.name)?
        .ok_or(miette::miette!("chain doesn't exist"))?;

    let db = Chain::load_db(&ctx.dirs.root_dir, &args.name)?;

    let mut upstream = Upstream::bootstrap(chain, db).await?;

    let span = info_span!("chain-sync");

    span.pb_set_style(
        &ProgressStyle::with_template("{spinner:.white} [{elapsed_precise}] {pos}/{len}").unwrap(),
    );

    let span = span.entered();

    let match_flag = AtomicBool::new(false);

    loop {
        upstream
            .next_step(|block| {
                if let Some(expected_tx_hash) = &args.tx_hash {
                    for tx in block.txs() {
                        let hash = tx.hash();
                        let hash = hex::encode(hash.as_ref());

                        if hash.eq(expected_tx_hash) {
                            info!("found tx hash");
                            match_flag.store(true, std::sync::atomic::Ordering::Relaxed);
                        }
                    }
                }
            })
            .await?;

        update_progress(
            &span,
            upstream.start_slot,
            upstream.current_slot,
            &upstream.tip,
        );

        if match_flag.load(std::sync::atomic::Ordering::Relaxed) == true {
            break;
        }
    }

    Ok(())
}
