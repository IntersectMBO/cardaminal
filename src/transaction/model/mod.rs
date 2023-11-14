use serde::{Deserialize, Serialize};

pub mod built;
pub mod serialise;
pub mod staging;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    #[default]
    Staging,
    Built,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Hash32([u8; 32]);

impl TryFrom<Vec<u8>> for Hash32 {
    type Error = miette::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let slice = <[u8; 32]>::try_from(value)
            .map_err(|_| miette::miette!("incorrect size for hash 28"))?;

        Ok(Hash32(slice))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Hash28(pub [u8; 28]);

impl TryFrom<Vec<u8>> for Hash28 {
    type Error = miette::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let slice = <[u8; 28]>::try_from(value)
            .map_err(|_| miette::miette!("incorrect size for hash 28"))?;

        Ok(Hash28(slice))
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Bytes(pub Vec<u8>);

impl Into<pallas::codec::utils::Bytes> for Bytes {
    fn into(self) -> pallas::codec::utils::Bytes {
        self.0.into()
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes(value)
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
