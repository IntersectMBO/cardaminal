use core::fmt;
use miette::{Context, IntoDiagnostic};
use pallas::ledger::addresses::Address as PallasAddress;
use serde::{
    de::{self, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{collections::HashMap, ops::Deref, str::FromStr};

use super::{Bytes, Hash28, Hash32, TxHash};

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct StagingTransaction {
    version: String,
    pub inputs: Option<Vec<Input>>,
    pub reference_inputs: Option<Vec<Input>>,
    pub outputs: Option<Vec<Output>>,
    pub fee: Option<u64>,
    pub mint: Option<MintAssets>,
    pub valid_from_slot: Option<u64>,
    pub invalid_from_slot: Option<u64>,
    pub network_id: Option<u32>,
    pub collateral_inputs: Option<Vec<Input>>,
    pub collateral_output: Option<CollateralOutput>,
    pub disclosed_signers: Option<Vec<PubKeyHash>>,
    pub scripts: Option<Vec<Script>>,
    pub datums: Option<Vec<DatumBytes>>,
    pub redeemers: Option<Redeemers>,
    pub signature_amount_override: Option<u8>,
    pub change_address: Option<Address>,
}
impl StagingTransaction {
    pub fn new() -> Self {
        Self {
            version: String::from("v1"),
            ..Default::default()
        }
    }
}

pub type PubKeyHash = Hash28;
pub type ScriptHash = Hash28;
pub type ScriptBytes = Bytes;
pub type PolicyId = ScriptHash;
pub type DatumBytes = Bytes;
pub type AssetName = Bytes;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Input {
    pub tx_hash: TxHash,
    pub tx_index: usize,
}
impl Input {
    pub fn new(tx_hash: TxHash, tx_index: usize) -> Self {
        Self { tx_hash, tx_index }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Output {
    pub address: Address,
    pub lovelace: u64,
    pub assets: Option<OutputAssets>,
    pub datum: Option<Datum>,
    pub script: Option<Script>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct OutputAssets(HashMap<PolicyId, HashMap<AssetName, u64>>);

impl TryFrom<Vec<String>> for OutputAssets {
    type Error = miette::ErrReport;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut assets: HashMap<PolicyId, HashMap<AssetName, u64>> = HashMap::new();
        for asset_string in value {
            let parts = asset_string.split(":").collect::<Vec<&str>>();
            if parts.len() != 3 {
                return Err(miette::ErrReport::msg("invalid asset string format"));
            }

            let policy = hex::decode(parts[0])
                .into_diagnostic()
                .context("parsing policy hex")?;

            let policy = Hash28(policy.try_into().unwrap());

            let asset = hex::decode(parts[1])
                .into_diagnostic()
                .context("parsing name hex")?;
            let asset = crate::transaction::model::Bytes(asset);

            let amount = parts[2]
                .parse::<u64>()
                .into_diagnostic()
                .context("parsing amount u64")?;

            assets
                .entry(policy)
                .and_modify(|policy_map| {
                    policy_map
                        .entry(asset.clone())
                        .and_modify(|asset_map| {
                            *asset_map += amount;
                        })
                        .or_insert(amount);
                })
                .or_insert_with(|| {
                    let mut map: HashMap<AssetName, u64> = HashMap::new();
                    map.insert(asset.clone(), amount);
                    map
                });
        }

        Ok(OutputAssets(assets))
    }
}

impl Serialize for OutputAssets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for (policy, assets) in self.0.iter() {
            let mut assets_map: HashMap<String, u64> = HashMap::new();

            for (asset, amount) in assets {
                assets_map.insert(hex::encode(&asset.0), *amount);
            }

            map.serialize_entry(policy, &assets_map)?;
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for OutputAssets {
    fn deserialize<D>(deserializer: D) -> Result<OutputAssets, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(OutputAssetsVisitor)
    }
}

struct OutputAssetsVisitor;

impl<'de> Visitor<'de> for OutputAssetsVisitor {
    type Value = OutputAssets;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "map of hex encoded policy ids to map of hex encoded asset names to u64 amounts",
        )
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut out_map = HashMap::new();

        while let Some((key, value)) = access.next_entry()? {
            out_map.insert(key, value);
        }

        Ok(OutputAssets(out_map))
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct MintAssets(pub HashMap<PolicyId, HashMap<AssetName, i64>>);

impl Serialize for MintAssets {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;

        for (policy, assets) in self.0.iter() {
            let mut assets_map: HashMap<String, i64> = HashMap::new();

            for (asset, amount) in assets {
                assets_map.insert(hex::encode(&asset.0), *amount);
            }

            map.serialize_entry(policy, &assets_map)?;
        }

        map.end()
    }
}

impl<'de> Deserialize<'de> for MintAssets {
    fn deserialize<D>(deserializer: D) -> Result<MintAssets, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MintAssetsVisitor)
    }
}

struct MintAssetsVisitor;

impl<'de> Visitor<'de> for MintAssetsVisitor {
    type Value = MintAssets;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "map of hex encoded policy ids to map of hex encoded asset names to u64 amounts",
        )
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut out_map = HashMap::new();

        while let Some((key, value)) = access.next_entry()? {
            out_map.insert(key, value);
        }

        Ok(MintAssets(out_map))
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CollateralOutput {
    pub address: Address,
    pub lovelace: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ScriptKind {
    Native,
    PlutusV1,
    PlutusV2,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Script {
    pub kind: ScriptKind,
    pub bytes: ScriptBytes,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DatumKind {
    Hash,
    Inline,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Datum {
    pub kind: DatumKind,
    pub bytes: DatumBytes,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum RedeemerPurpose {
    Spend(TxHash, usize),
    Mint(PolicyId),
    // Reward TODO
    // Cert TODO
}

impl Serialize for RedeemerPurpose {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str = match self {
            RedeemerPurpose::Spend(hash, index) => {
                format!("spend:{}#{}", hex::encode(&hash.0), index)
            }
            RedeemerPurpose::Mint(hash) => format!("mint:{}", hex::encode(&hash.0)),
        };

        serializer.serialize_str(&str)
    }
}

impl<'de> Deserialize<'de> for RedeemerPurpose {
    fn deserialize<D>(deserializer: D) -> Result<RedeemerPurpose, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RedeemerPurposeVisitor)
    }
}

struct RedeemerPurposeVisitor;

impl<'de> Visitor<'de> for RedeemerPurposeVisitor {
    type Value = RedeemerPurpose;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("'spend:{hex_txid}#{index}' or 'mint:{hex_policyid}'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let (tag, item) = v
            .split_once(":")
            .ok_or(E::custom("invalid redeemer purpose"))?;

        match tag {
            "spend" => {
                let (hash, index) = item
                    .split_once("#")
                    .ok_or(E::custom("invalid spend redeemer item"))?;

                let hash = Hash32(
                    hex::decode(hash)
                        .map_err(|_| E::custom("invalid spend redeemer item txid hex"))?
                        .try_into()
                        .map_err(|_| E::custom("invalid spend redeemer txid len"))?,
                );
                let index = index
                    .parse()
                    .map_err(|_| E::custom("invalid spend redeemer item index"))?;

                Ok(RedeemerPurpose::Spend(hash, index))
            }
            "mint" => {
                let hash = Hash28(
                    hex::decode(item)
                        .map_err(|_| E::custom("invalid mint redeemer item policy hex"))?
                        .try_into()
                        .map_err(|_| E::custom("invalid mint redeemer policy len"))?,
                );

                Ok(RedeemerPurpose::Mint(hash))
            }
            _ => Err(E::custom("invalid redeemer tag")),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct ExUnits {
    mem: u64,
    steps: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Redeemers(HashMap<RedeemerPurpose, Option<ExUnits>>);

#[derive(PartialEq, Eq, Debug)]
pub struct Address(PallasAddress);

impl Deref for Address {
    type Target = PallasAddress;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PallasAddress> for Address {
    fn from(value: PallasAddress) -> Self {
        Self(value)
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AddressVisitor)
    }
}

struct AddressVisitor;

impl<'de> Visitor<'de> for AddressVisitor {
    type Value = Address;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("bech32 shelley address or base58 byron address")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Address(
            PallasAddress::from_str(v).map_err(|_| E::custom("invalid address"))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pallas::ledger::addresses::Address as PallasAddress;

    use super::*;

    #[test]
    fn json_roundtrip() {
        let tx = StagingTransaction {
            version: String::from("v1"),
            inputs: Some(
                vec![
                    Input {
                        tx_hash: Hash32([0; 32]),
                        tx_index: 1
                    }
                ]
            ) ,
            reference_inputs: Some(vec![
                Input {
                    tx_hash: Hash32([1; 32]),
                    tx_index: 0
                }
            ]),
            outputs: Some(vec![
                Output {
                    address: Address(PallasAddress::from_str("addr1g9ekml92qyvzrjmawxkh64r2w5xr6mg9ngfmxh2khsmdrcudevsft64mf887333adamant").unwrap()),
                    lovelace: 1337,
                    assets: Some(
                        OutputAssets(
                            vec![
                                (
                                    Hash28([0; 28]),
                                    (vec![(Bytes(vec![0]), 1337)]).into_iter().collect::<HashMap<_, _>>()
                                )
                            ].into_iter().collect::<HashMap<_, _>>()
                        )
                    ),
                    datum: Some(Datum { kind: DatumKind::Hash, bytes: Bytes([0; 32].to_vec()) }),
                    script: Some(Script { kind: ScriptKind::Native, bytes: Bytes([1; 100].to_vec()) }),
                }
            ]),
            fee: Some(1337),
            mint: Some(
                MintAssets(
                    vec![
                        (
                            Hash28([0; 28]),
                            (vec![(Bytes(vec![0]), -1337)]).into_iter().collect::<HashMap<_, _>>()
                        )
                    ].into_iter().collect::<HashMap<_, _>>()
                )
            ),
            valid_from_slot: Some(1337),
            invalid_from_slot: Some(1337),
            network_id: Some(1),
            collateral_inputs: Some(vec![
                Input {
                    tx_hash: Hash32([2; 32]),
                    tx_index: 0
                }
            ]),
            collateral_output: Some(CollateralOutput { address: Address(PallasAddress::from_str("addr1g9ekml92qyvzrjmawxkh64r2w5xr6mg9ngfmxh2khsmdrcudevsft64mf887333adamant").unwrap()), lovelace: 1337 }),
            disclosed_signers: Some(vec![Hash28([0; 28])]),
            scripts: Some(vec![
                Script { kind: ScriptKind::PlutusV1, bytes: Bytes([0; 100].to_vec()) }
            ]),
            datums: Some(vec![Bytes([0; 100].to_vec())]),
            redeemers: Some(Redeemers(vec![
                (RedeemerPurpose::Spend(Hash32([4; 32]), 0), Some(ExUnits { mem: 1337, steps: 7331 })),
                (RedeemerPurpose::Mint(Hash28([5; 28])), None),
            ].into_iter().collect::<HashMap<_, _>>())),
            signature_amount_override: Some(5),
            change_address: Some(Address(PallasAddress::from_str("addr1g9ekml92qyvzrjmawxkh64r2w5xr6mg9ngfmxh2khsmdrcudevsft64mf887333adamant").unwrap())),
        };

        let serialised_tx = serde_json::to_string(&tx).unwrap();
        dbg!(&serialised_tx);

        let deserialised_tx: StagingTransaction = serde_json::from_str(&serialised_tx).unwrap();

        assert_eq!(tx, deserialised_tx)
    }
}
