use clap::Parser;
use tracing::{info, instrument};

#[derive(Parser)]
pub struct Args {
    /// name of the chain to delete
    name: String,

    /// automatically detach any wallets using this chain
    #[arg(long, default_value_t)]
    detach: bool,
}

#[instrument("delete", skip_all, fields(name=args.name))]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("deleting");

    for i in 0..3 {
        info!(wallet = i, "detached");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }

    info!("chain deleted");

    Ok(())
}
