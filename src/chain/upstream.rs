use miette::{Context, IntoDiagnostic};
use pallas::{
    ledger::traverse::MultiEraHeader,
    network::{
        facades::PeerClient,
        miniprotocols::{
            chainsync::{NextResponse, Tip},
            Point,
        },
    },
    storage::rolldb::chain,
};
use tracing::{info, warn};

use super::config::Chain;

pub struct Upstream {
    peer_client: PeerClient,
    db: chain::Store,

    pub start_slot: u64,
    pub current_slot: Option<u64>,
    pub current_block: Option<Vec<u8>>,
    pub tip: Option<Tip>,
    pub is_tip: bool,
}

impl Upstream {
    pub async fn bootstrap(chain: Chain, db: chain::Store) -> miette::Result<Self> {
        let mut points: Vec<_> = db
            .intersect_options(5)
            .into_diagnostic()
            .context("looking for intersect points")?
            .iter()
            .map(|(s, h)| Point::Specific(*s, h.to_vec()))
            .collect();

        // if we have no intersection points, it means the chain db is empty and this is
        // the first time trying to sync. We need to check if there's some `after` value
        // set in config and, in that case, avoid starting from origin.
        if points.is_empty() {
            if let Some(after) = chain.after {
                points = vec![Point::Specific(after.slot, after.hash.to_vec())];
            }
        }

        info!(?points, "intersecting chain");

        let magic: u64 = chain.magic.parse().into_diagnostic()?;

        let mut peer_client = PeerClient::connect(&chain.upstream.address, magic)
            .await
            .into_diagnostic()?;

        let start_slot;

        if points.is_empty() {
            let point = peer_client
                .chainsync()
                .intersect_origin()
                .await
                .into_diagnostic()?;

            start_slot = point.slot_or_default();
        } else {
            let (point, _) = peer_client
                .chainsync()
                .find_intersect(points)
                .await
                .into_diagnostic()?;

            start_slot = point.unwrap_or(Point::Origin).slot_or_default();
        }

        let out = Self {
            peer_client,
            db,
            start_slot,
            current_slot: None,
            current_block: None,
            tip: None,
            is_tip: false,
        };

        Ok(out)
    }

    pub async fn next_step(&mut self) -> miette::Result<()> {
        let response = self
            .peer_client
            .chainsync()
            .request_next()
            .await
            .into_diagnostic()?;

        match response {
            NextResponse::RollForward(header, tip) => {
                let header = match header.byron_prefix {
                    Some((subtag, _)) => {
                        MultiEraHeader::decode(header.variant, Some(subtag), &header.cbor)
                    }
                    None => MultiEraHeader::decode(header.variant, None, &header.cbor),
                }
                .into_diagnostic()?;

                let slot = header.slot();
                let hash = header.hash();

                let block = self
                    .peer_client
                    .blockfetch()
                    .fetch_single(Point::Specific(slot, hash.to_vec()))
                    .await
                    .into_diagnostic()
                    .context("error fetching block from upstream peer")?;

                self.db
                    .roll_forward(header.slot(), header.hash(), block.clone())
                    .into_diagnostic()
                    .context("error saving block to db")?;

                self.is_tip = false;
                self.tip = Some(tip);
                self.current_slot = Some(slot);
                self.current_block = Some(block);

                info!(last_slot = slot, "new blocks downloaded");
            }
            NextResponse::RollBackward(point, tip) => {
                match point {
                    Point::Origin => {
                        self.db
                            .roll_back_origin()
                            .into_diagnostic()
                            .context("error saving block to db")?;

                        self.start_slot = 0;
                        self.is_tip = false;
                        self.tip = Some(tip);
                        self.current_slot = None;
                        self.current_block = None;

                        info!("rolled back to origin");
                    }
                    Point::Specific(slot, _hash) => {
                        //let hash = Hash::<32>::from(&hash[0..8]);
                        self.db
                            .roll_back(slot)
                            .into_diagnostic()
                            .context("error saving block to db")?;

                        self.start_slot = self.start_slot.min(slot);
                        self.is_tip = false;
                        self.tip = Some(tip);
                        self.current_slot = Some(slot);
                        self.current_block = None;

                        warn!(slot, "rolled back to slot");
                    }
                }
            }
            NextResponse::Await => {
                self.is_tip = true;
                self.current_block = None;

                warn!("reached tip of the chain");
            }
        };

        Ok(())
    }
}
