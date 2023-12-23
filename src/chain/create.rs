use std::{fs, io::Write};

use clap::Parser;
use miette::{bail, IntoDiagnostic};
use pallas::{
    crypto::hash::Hash,
    network::{facades::PeerClient, miniprotocols::Point},
};
use tracing::{info, instrument};

use crate::chain::config::{Chain, ChainUpstream};

use super::config::ChainAfter;

#[derive(Parser)]
pub struct Args {
    /// friendly name to identify the chain
    pub name: String,

    /// [host]:[port] of the upstream node
    pub upstream: String,

    /// network magic of the chain
    pub magic: String,

    /// network id for addresses
    pub address_network_id: u8,

    /// [slot],[hash] of the sync start point
    #[arg(short, long)]
    pub after: Option<String>,

    #[arg(long, action)]
    pub after_tip: bool,
}

#[instrument("create", skip_all, fields(name=args.name))]
pub async fn run(args: Args, ctx: &crate::Context) -> miette::Result<()> {
    let after = match (args.after, args.after_tip) {
        (Some(after), _) => ChainAfter::try_from(after)?.into(),
        (_, true) => find_tip(&args.upstream, &args.magic).await?.into(),
        _ => None,
    };

    let chain_slug = slug::slugify(&args.name);

    let chain = Chain::new(
        args.name,
        args.magic,
        args.address_network_id,
        ChainUpstream {
            address: args.upstream,
        },
        after,
    );

    let chain_path = ctx.dirs.root_dir.join("chains").join(&chain_slug);

    if chain_path.exists() {
        bail!("chain already exists")
    }

    fs::create_dir_all(&chain_path).into_diagnostic()?;

    let toml_string = toml::to_string(&chain).into_diagnostic()?;
    let mut file = fs::File::create(chain_path.join("config.toml")).into_diagnostic()?;
    file.write_all(toml_string.as_bytes()).into_diagnostic()?;

    info!(chain = chain_slug, "chain created");

    Ok(())
}

async fn find_tip(upstream: &str, magic: &str) -> miette::Result<ChainAfter> {
    info!("querying chain tip from upstream node");

    let magic = magic.parse().into_diagnostic()?;

    let mut peer_client = PeerClient::connect(upstream, magic)
        .await
        .into_diagnostic()?;

    let point = peer_client
        .chainsync()
        .intersect_tip()
        .await
        .into_diagnostic()?;

    let after = match point {
        Point::Origin => bail!("can't find tip if chain hasn't started"),
        Point::Specific(slot, hash) => {
            let hash = Hash::<32>::from(hash.as_slice());
            ChainAfter::new(slot, hash)
        }
    };

    info!(slot=after.slot, hash=%after.hash, "found chain tip");

    peer_client.abort().await;

    Ok(after)
}
