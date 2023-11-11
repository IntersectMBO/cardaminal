use core::fmt;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

pub mod staging;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Hash32([u8; 32]);

impl Serialize for Hash32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(self.0))
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

impl TryFrom<Vec<u8>> for Hash32 {
    type Error = miette::ErrReport;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .as_slice()
                .try_into()
                .map_err(|_| miette::ErrReport::msg("hash malformed"))?,
        ))
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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Hash28(pub [u8; 28]);

impl Serialize for Hash28 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode(self.0))
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

impl TryFrom<Vec<u8>> for Hash28 {
    type Error = miette::ErrReport;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Ok(Self(
            value
                .as_slice()
                .try_into()
                .map_err(|_| miette::ErrReport::msg("hash malformed"))?,
        ))
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

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Bytes(pub Vec<u8>);

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

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes(value)
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

pub type TxHash = Hash32;
impl TryFrom<String> for TxHash {
    type Error = miette::ErrReport;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Hash32(
            hex::decode(value)
                .map_err(|_| miette::miette!("invalid hex"))?
                .try_into()
                .map_err(|_| miette::miette!("invalid length"))?,
        ))
    }
}
