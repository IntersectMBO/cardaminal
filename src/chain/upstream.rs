use miette::{Context, IntoDiagnostic};
use pallas::{
    ledger::traverse::{MultiEraBlock, MultiEraHeader},
    network::{
        facades::PeerClient,
        miniprotocols::{
            chainsync::{NextResponse, RollbackBuffer, Tip},
            Point,
        },
    },
    storage::rolldb::chain,
};
use tracing::{info, warn};

use super::config::Chain;

const FETCH_BATCH_SIZE: usize = 10;

pub struct Upstream {
    peer_client: PeerClient,
    db: chain::Store,
    buffer: pallas::network::miniprotocols::chainsync::RollbackBuffer,

    pub start_slot: u64,
    pub current_slot: Option<u64>,
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
            buffer: RollbackBuffer::new(),
            start_slot,
            current_slot: None,
            tip: None,
            is_tip: false,
        };

        Ok(out)
    }

    async fn fetch_blocks<B>(&mut self, block_inspector: B) -> miette::Result<()>
    where
        B: Fn(&MultiEraBlock) -> (),
    {
        let oldest = self.buffer.oldest().unwrap();
        let latest = self.buffer.latest().unwrap();

        let blocks = self
            .peer_client
            .blockfetch()
            .fetch_range((oldest.clone(), latest.clone()))
            .await
            .into_diagnostic()
            .context("error fetching block from upstream peer")?;

        for cbor in blocks {
            let block = MultiEraBlock::decode(&cbor)
                .into_diagnostic()
                .context("decoding block cbor")?;

            self.db
                .roll_forward(block.slot(), block.hash(), cbor.clone())
                .into_diagnostic()
                .context("error saving block to db")?;

            block_inspector(&block);
        }

        info!(
            oldest = oldest.slot_or_default(),
            latest = latest.slot_or_default(),
            "downloaded block range"
        );

        self.buffer = RollbackBuffer::new();

        Ok(())
    }

    pub async fn roll_chain(&mut self) -> miette::Result<()> {
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

                self.buffer
                    .roll_forward(Point::Specific(slot, hash.to_vec()));
                self.is_tip = false;
                self.tip = Some(tip);
                self.current_slot = Some(slot);

                info!(slot, "chain roll forward");
            }
            NextResponse::RollBackward(point, tip) => {
                match point {
                    Point::Origin => {
                        self.db
                            .roll_back_origin()
                            .into_diagnostic()
                            .context("error saving block to db")?;

                        self.buffer = RollbackBuffer::new();
                        self.start_slot = 0;
                        self.is_tip = false;
                        self.tip = Some(tip);
                        self.current_slot = None;

                        info!("chain rolled back to origin");
                    }
                    Point::Specific(slot, hash) => {
                        //let hash = Hash::<32>::from(&hash[0..8]);
                        self.db
                            .roll_back(slot)
                            .into_diagnostic()
                            .context("error saving block to db")?;

                        self.buffer.roll_back(&Point::Specific(slot, hash));
                        self.start_slot = self.start_slot.min(slot);
                        self.is_tip = false;
                        self.tip = Some(tip);
                        self.current_slot = Some(slot);

                        warn!(slot, "chain rolled back");
                    }
                }
            }
            NextResponse::Await => {
                self.is_tip = true;

                warn!("reached tip of the chain");
            }
        };

        Ok(())
    }

    pub async fn next_step<B>(&mut self, block_inspector: B) -> miette::Result<()>
    where
        B: Fn(&MultiEraBlock) -> (),
    {
        if self.buffer.size() < FETCH_BATCH_SIZE {
            self.roll_chain().await
        } else {
            self.fetch_blocks(block_inspector).await
        }
    }
}
