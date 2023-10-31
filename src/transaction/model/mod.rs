use serde::{Deserialize, Serialize};

pub mod built;
pub mod serialise;
pub mod staging;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TransactionStatus {
    Staging,
    Built,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Hash32([u8; 32]);

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Hash28([u8; 28]);

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Bytes(Vec<u8>);

type TxHash = Hash32;
