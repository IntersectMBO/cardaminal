use clap::Parser;
use pallas::{ledger::traverse::MultiEraHeader, network::miniprotocols::chainsync::NextResponse};
use tracing::{info, instrument};

use crate::sources::n2n::bootstrap;

#[derive(Parser)]
pub struct Args {
    /// Name of the chain to sync
    #[arg(short, long)]
    name: String,
}

#[instrument("update", skip_all)]
pub async fn run(args: Args) -> miette::Result<()> {
    info!("Updating chain {}", args.name);

    //TODO: get chain config from sqlite
    //TODO: validate errors

    let mut peer_client = bootstrap(
        "relaysnew.cardano-mainnet.iohk.io:3001",
        &764824073,
        crate::sources::IntersectConfig::Origin,
    )
    .await
    .unwrap();

    let chainsync = peer_client.chainsync();

    while chainsync.has_agency() {
        let response = chainsync.request_next().await.unwrap();

        match response {
            NextResponse::RollForward(header, _) => {
                let header = match header.byron_prefix {
                    Some((subtag, _)) => {
                        MultiEraHeader::decode(header.variant, Some(subtag), &header.cbor)
                    }
                    None => MultiEraHeader::decode(header.variant, None, &header.cbor),
                }
                .unwrap();

                let slot = header.slot();
                let hash = header.hash();

                info!("chain sync roll forward slot={} hash={}", slot, hash);
            }
            NextResponse::RollBackward(point, _) => {
                info!("chain sync roll backward slot={}", point.slot_or_default());
            }
            NextResponse::Await => break,
        }
    }

    info!("Chain updated");

    Ok(())
}
