use std::{thread, time::Duration};

use clap::Parser;
use indicatif::ProgressStyle;
use tracing::{info, info_span, instrument, Span};
use tracing_indicatif::span_ext::IndicatifSpanExt;

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to update
    name: String,
}

#[instrument("update", skip_all, fields(name=args.name))]
pub async fn run(args: Args) -> miette::Result<()> {
    info!(chain = args.name, "updating");

    let mut slot = 0;
    let slot_tip = 500;

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
        slot += 1;

        info!(last_slot = slot, "new blocks downloaded");
        thread::sleep(Duration::from_millis(500));
        Span::current().pb_inc(1);
    }

    std::mem::drop(span_enter);
    std::mem::drop(span);

    info!("chain updated");

    Ok(())
}
