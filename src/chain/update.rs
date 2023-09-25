use std::{fmt::Write, thread, time::Duration};

use clap::Parser;
use indicatif::{ProgressState, ProgressStyle};
use tracing::{info, info_span, instrument, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to update
    name: String,
}

#[instrument]
async fn do_work(slot: u64) {
    thread::sleep(Duration::from_millis(12));
}

#[instrument("update", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!(chain = args.name, "updating");

    let mut slot = 0;
    let slot_tip = 500;

    let header_span = info_span!("header");
    header_span.pb_set_style(&ProgressStyle::default_bar());
    header_span.pb_set_length(slot_tip);

    header_span.pb_set_style(
        &ProgressStyle::with_template(
            "{spinner:.white} [{elapsed_precise}] [{wide_bar:.white/white}] {pos}/{len}",
        )
        .unwrap(),
    );

    let header_span_enter = header_span.enter();

    while slot < slot_tip {
        slot += 1;
        do_work(slot).await;
        Span::current().pb_inc(1);
    }

    std::mem::drop(header_span_enter);
    std::mem::drop(header_span);

    info!("chain updated");

    Ok(())
}
