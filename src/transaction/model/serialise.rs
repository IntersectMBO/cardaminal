use core::fmt;
use std::{collections::HashMap, str::FromStr};

use pallas::ledger::addresses::Address as PallasAddress;
use serde::{
    de::{self, Visitor},
    ser::SerializeMap,
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::{
    built::Bytes64,
    staging::{Address, MintAssets, OutputAssets, RedeemerPurpose},
    Bytes, Hash28, Hash32,
};

impl Serialize for Hash32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Hash32 {
    fn deserialize<D>(deserializer: D) -> Result<Hash32, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Hash32Visitor)
    }
}

struct Hash32Visitor;

impl<'de> Visitor<'de> for Hash32Visitor {
    type Value = Hash32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("32 bytes hex encoded")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Hash32(
            hex::decode(v)
                .map_err(|_| E::custom("invalid hex"))?
                .try_into()
                .map_err(|_| E::custom("invalid length"))?,
        ))
    }
}

impl Serialize for Hash28 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Hash28 {
    fn deserialize<D>(deserializer: D) -> Result<Hash28, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Hash28Visitor)
    }
}

struct Hash28Visitor;

impl<'de> Visitor<'de> for Hash28Visitor {
    type Value = Hash28;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("28 bytes hex encoded")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Hash28(
            hex::decode(v)
                .map_err(|_| E::custom("invalid hex"))?
                .try_into()
                .map_err(|_| E::custom("invalid length"))?,
        ))
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Bytes, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(BytesVisitor)
    }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("bytes hex encoded")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Bytes(hex::decode(v).map_err(|_| E::custom("invalid hex"))?))
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

impl Serialize for Bytes64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Bytes64 {
    fn deserialize<D>(deserializer: D) -> Result<Bytes64, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Bytes64Visitor)
    }
}

struct Bytes64Visitor;

impl<'de> Visitor<'de> for Bytes64Visitor {
    type Value = Bytes64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("64 bytes hex encoded")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Bytes64(
            hex::decode(v)
                .map_err(|_| E::custom("invalid hex"))?
                .try_into()
                .map_err(|_| E::custom("invalid length"))?,
        ))
    }
}
