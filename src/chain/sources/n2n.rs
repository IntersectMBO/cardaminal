use miette::IntoDiagnostic;
use pallas::network::{facades::PeerClient, miniprotocols::Point};

use super::IntersectConfig;

pub async fn bootstrap(
    peer_address: &str,
    magic: &u64,
    intersect: IntersectConfig,
) -> miette::Result<PeerClient> {
    let mut peer_client = PeerClient::connect(peer_address, *magic)
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
