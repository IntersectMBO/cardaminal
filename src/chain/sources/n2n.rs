use miette::IntoDiagnostic;
use pallas::network::{facades::PeerClient, miniprotocols::Point};

use crate::chain::config::Chain;

use super::IntersectConfig;

pub async fn bootstrap(chain: &Chain, intersect: IntersectConfig) -> miette::Result<PeerClient> {
    let magic: u64 = chain.magic.parse().into_diagnostic()?;
    let mut peer_client = PeerClient::connect(&chain.upstream.address, magic)
        .await
        .into_diagnostic()?;

    let chainsync = peer_client.chainsync();
    match intersect {
        IntersectConfig::Origin => {
            chainsync.intersect_origin().await.into_diagnostic()?;
        }
        IntersectConfig::Point(slot, hash) => {
            let hash = hex::decode(hash).expect("invalid hex hash");
            let point = Point::Specific(slot, hash);
            chainsync
                .find_intersect(vec![point])
                .await
                .into_diagnostic()?;
        }
    }

    Ok(peer_client)
}
