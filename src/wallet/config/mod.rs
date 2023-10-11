use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::create::Args;
use crate::utils::{deserialize_date, serialize_date};

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub version: String,
    pub name: String,
    pub chain: Option<String>,

    #[serde(serialize_with = "serialize_date")]
    #[serde(deserialize_with = "deserialize_date")]
    pub created_on: DateTime<Local>,
}
impl Wallet {
    pub fn new(name: String, chain: Option<String>) -> Self {
        let version = String::from("v1alpha");

        Self {
            version,
            name,
            chain,
            created_on: Local::now(),
        }
    }

    pub fn dir(root_dir: &Path, name: &str) -> PathBuf {
        root_dir.join("wallets").join(name)
    }
}
impl From<&Args> for Wallet {
    fn from(value: &Args) -> Self {
        Self::new(value.name.clone(), value.chain.clone())
    }
}
