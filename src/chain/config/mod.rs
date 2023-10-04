use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use comfy_table::Table;
use miette::{Context, IntoDiagnostic};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

use super::create::Args;

const DATE_FORMAT: &str = "%Y-%m-%d";

#[derive(Serialize, Deserialize)]
pub struct Chain {
    version: String,
    name: String,
    upstream: Vec<ChainUpstream>,
    magic: String,
    after: Option<ChainAfter>,

    #[serde(serialize_with = "serialize_date")]
    #[serde(deserialize_with = "deserialize_date")]
    created_on: DateTime<Local>,
}
impl Chain {
    pub fn try_new(
        name: String,
        magic: String,
        upstream: Vec<ChainUpstream>,
        after: Option<String>,
    ) -> miette::Result<Self> {
        // TODO: Get cli version
        let version = String::from("v1alpha");

        let mut chain = Self {
            version,
            name,
            upstream,
            magic,
            after: None,
            created_on: Local::now(),
        };

        if let Some(after) = after {
            chain.after = Some(after.try_into()?);
        }

        Ok(chain)
    }
}
impl TryFrom<&Args> for Chain {
    type Error = miette::ErrReport;

    fn try_from(value: &Args) -> Result<Self, Self::Error> {
        let chain_upstream = ChainUpstream::new(value.upstream.clone());
        Ok(Self::try_new(
            value.name.clone(),
            value.magic.clone(),
            vec![chain_upstream],
            value.after.clone(),
        )?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChainUpstream {
    address: String,
}
impl ChainUpstream {
    pub fn new(address: String) -> Self {
        Self { address }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ChainAfter {
    slot: u64,
    hash: String,
}
impl ChainAfter {
    pub fn new(slot: u64, hash: String) -> Self {
        Self { slot, hash }
    }
}
impl TryFrom<String> for ChainAfter {
    type Error = miette::ErrReport;

    fn try_from(value: String) -> Result<Self, Self::Error> {
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
    let date_str = date.format(DATE_FORMAT).to_string();
    serializer.serialize_str(&date_str)
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: String = Deserialize::deserialize(deserializer)?;

    let parsed_date =
        NaiveDate::parse_from_str(&date_str, DATE_FORMAT).map_err(de::Error::custom)?;

    let local_date = Local
        .from_local_datetime(&NaiveDateTime::new(parsed_date, NaiveTime::default()))
        .unwrap();

    Ok(local_date)
}

// TODO: validate if other structs could use this trait and if yes, will change to global formatter trait
pub trait ChainFormatter {
    fn to_table(&self);
    fn to_json(&self);
}

impl ChainFormatter for Vec<Chain> {
    fn to_table(&self) {
        let mut table = Table::new();

        table.set_header(vec!["name", "upstream", "magic"]);

        for chain in self {
            let upstream = chain
                .upstream
                .iter()
                .map(|u| u.address.clone())
                .collect::<Vec<String>>()
                .join(",");

            table.add_row(vec![&chain.name, &upstream, &chain.magic]);
        }

        println!("{table}");
    }

    fn to_json(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        println!("{json}");
    }
}
