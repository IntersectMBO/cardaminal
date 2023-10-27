use core::fmt;
use pallas::ledger::addresses::Address as PallasAddress;
use std::{collections::HashMap, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{
    de::{self, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::{Bytes, Hash28, Hash32, TransactionStatus, TxHash};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct StagingTransaction {
    version: u8,
    created_at: DateTime<Utc>,
    status: TransactionStatus,
    inputs: Vec<Input>,
    reference_inputs: Option<Vec<Input>>,
    outputs: Option<Vec<Output>>,
    fee: Option<u64>,
    mint: Option<MintAssets>,
    valid_from_slot: Option<u64>,
    invalid_from_slot: Option<u64>,
    network_id: Option<u32>,
    collateral_inputs: Option<Vec<Input>>,
    collateral_output: Option<CollateralOutput>,
    disclosed_signers: Option<Vec<PubKeyHash>>,
    scripts: Option<Vec<Script>>,
    datums: Option<Vec<DatumBytes>>,
    redeemers: Option<Redeemers>,
    signature_amount_override: Option<u8>,
    change_address: Option<Address>,
}

type PubKeyHash = Hash28;
type ScriptHash = Hash28;
type ScriptBytes = Bytes;
type PolicyId = ScriptHash;
type DatumBytes = Bytes;
type AssetName = Bytes;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Input {
    tx_hash: TxHash,
    tx_index: usize,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Output {
    address: Address,
    lovelace: u64,
    assets: Option<OutputAssets>,
    datum: Option<Datum>,
    script: Option<Script>,
}

#[derive(PartialEq, Eq, Debug)]
struct OutputAssets(HashMap<PolicyId, HashMap<AssetName, u64>>);

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
struct MintAssets(HashMap<PolicyId, HashMap<AssetName, i64>>);

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
struct CollateralOutput {
    address: Address,
    lovelace: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
enum ScriptKind {
    Native,
    PlutusV1,
    PlutusV2,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Script {
    kind: ScriptKind,
    bytes: ScriptBytes,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
enum DatumKind {
    Hash,
    Inline,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Datum {
    kind: DatumKind,
    bytes: DatumBytes,
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
struct Redeemers(HashMap<RedeemerPurpose, Option<ExUnits>>);

#[derive(PartialEq, Eq, Debug)]
struct Address(PallasAddress);

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

    use chrono::DateTime;
    use pallas::ledger::addresses::Address as PallasAddress;

    use super::*;

    #[test]
    fn json_roundtrip() {
        let tx = StagingTransaction {
            version: 3,
            created_at: DateTime::from_timestamp(0, 0).unwrap(),
            status: TransactionStatus::Staging,
            inputs: vec![
                Input {
                    tx_hash: Hash32([0; 32]),
                    tx_index: 1
                }
            ],
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

        let deserialised_tx: StagingTransaction = serde_json::from_str(&serialised_tx).unwrap();

        assert_eq!(tx, deserialised_tx)
    }
}
