use pallas::network::{facades::PeerClient, miniprotocols::Point};

use crate::errors::Error;

use super::IntersectConfig;

pub async fn bootstrap(
    peer_address: &str,
    magic: &u64,
    intersect: IntersectConfig,
) -> Result<PeerClient, Error> {
    let mut peer_client = PeerClient::connect(peer_address, magic.clone()).await?;

    let chainsync = peer_client.chainsync();
    match intersect {
        IntersectConfig::Origin => {
            chainsync.intersect_origin().await?;
        }
        IntersectConfig::Point(slot, hash) => {
            let hash = hex::decode(hash).expect("invalid hex hash");
            let point = Point::Specific(slot, hash);
            chainsync.find_intersect(vec![point]).await?;
        }
    }

    Ok(peer_client)
}
