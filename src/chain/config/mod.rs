use chrono::{DateTime, Local};
use miette::{Context, IntoDiagnostic};
use serde::{Serialize, Serializer};

use super::create::Args;

#[derive(Serialize)]
pub struct Chain {
    version: String,
    name: String,
    upstream: Vec<ChainUpstream>,
    magic: String,
    after: Option<ChainAfter>,

    #[serde(serialize_with = "serialize_date")]
    created_on: DateTime<Local>,
}
impl Chain {
    pub fn new(name: String, magic: String, upstream: Vec<ChainUpstream>) -> Self {
        let version = String::from("v1alpha");

        Self {
            version,
            name: name.clone(),
            upstream,
            magic: magic.clone(),
            after: None,
            created_on: Local::now(),
        }
    }

    pub fn set_after(&mut self, after: ChainAfter) {
        self.after = Some(after);
    }
}
impl TryFrom<&Args> for Chain {
    type Error = miette::ErrReport;

    fn try_from(value: &Args) -> Result<Self, Self::Error> {
        let chain_upstream = ChainUpstream::new(value.upstream.clone());
        let mut chain = Self::new(
            value.name.clone(),
            value.magic.clone(),
            vec![chain_upstream],
        );

        if let Some(after) = &value.after {
            chain.set_after(after.try_into()?)
        }

        Ok(chain)
    }
}

#[derive(Serialize)]
pub struct ChainUpstream {
    address: String,
}
impl ChainUpstream {
    pub fn new(address: String) -> Self {
        Self { address }
    }
}

#[derive(Serialize)]
pub struct ChainAfter {
    slot: u64,
    hash: String,
}
impl ChainAfter {
    pub fn new(slot: u64, hash: String) -> Self {
        Self { slot, hash }
    }
}
impl TryFrom<&String> for ChainAfter {
    type Error = miette::ErrReport;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let parts = value.split(",").collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(miette::ErrReport::msg("invalid after format"));
        }

        let slot = parts[0]
            .parse::<u64>()
            .into_diagnostic()
            .with_context(|| "After slot must be u64")?;

        let hash = parts[1];

        Ok(ChainAfter::new(slot, hash.to_string()))
    }
}

fn serialize_date<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date_str = date.format("%Y-%m-%d").to_string();
    serializer.serialize_str(&date_str)
}
