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
pub struct Bytes32(pub [u8; 32]);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Hash28(pub [u8; 28]);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Bytes(pub Vec<u8>);

impl Into<pallas::codec::utils::Bytes> for Bytes {
    fn into(self) -> pallas::codec::utils::Bytes {
        self.0.into()
    }
}

pub type TxHash = Bytes32;
impl TryFrom<String> for TxHash {
    type Error = miette::ErrReport;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Bytes32(
            hex::decode(value)
                .map_err(|_| miette::miette!("invalid hex"))?
                .try_into()
                .map_err(|_| miette::miette!("invalid length"))?,
        ))
    }
}
