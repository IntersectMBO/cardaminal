use std::{
    fs,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use chrono::{DateTime, Local};
use comfy_table::Table;
use miette::{Context, IntoDiagnostic};
use serde::{Deserialize, Serialize};

use super::create::Args;
use crate::utils::{deserialize_date, serialize_date, OutputFormatter};

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
        // TODO: Get cli version
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

    pub fn load_config(root_dir: &Path, name: &str) -> miette::Result<Option<Self>> {
        let config_path = Self::config_path(root_dir, name);

        if config_path.exists() {
            let file = fs::File::open(config_path).into_diagnostic()?;
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).into_diagnostic()?;

            let wallet: Wallet = toml::from_str(&contents).into_diagnostic()?;
            return Ok(Some(wallet));
        }

        Ok(None)
    }

    pub fn config_path(root_dir: &Path, name: &str) -> PathBuf {
        Self::dir(root_dir, name).join("config.toml")
    }

    pub fn list_available(root_dir: &Path) -> miette::Result<Vec<String>> {
        let parent = root_dir
            .join("wallets")
            .read_dir()
            .into_diagnostic()
            .context("can't read wallet parent dir")?;

        let names = parent
            .into_iter()
            .filter_map(|dir| dir.ok())
            .map(|d| String::from(d.file_name().to_string_lossy()))
            .collect();

        Ok(names)
    }

    pub fn save_config(&self, root_dir: &Path) -> miette::Result<()> {
        let config_path = Self::config_path(root_dir, &self.name);
        let toml_string = toml::to_string(self).into_diagnostic()?;
        let mut file = fs::File::create(config_path).into_diagnostic()?;
        file.write_all(toml_string.as_bytes()).into_diagnostic()?;
        Ok(())
    }
}
impl From<&Args> for Wallet {
    fn from(value: &Args) -> Self {
        Self::new(value.name.clone(), value.chain.clone())
    }
}

impl OutputFormatter for Vec<Wallet> {
    fn to_table(&self) {
        let mut table = Table::new();

        table.set_header(vec!["name", "chain"]);

        for wallet in self {
            table.add_row(vec![
                &wallet.name,
                wallet
                    .chain
                    .as_ref()
                    .unwrap_or(&String::from("not attached")),
            ]);
        }

        println!("{table}");
    }

    fn to_json(&self) {
        let json = serde_json::to_string_pretty(self).unwrap();
        println!("{json}");
    }
}
