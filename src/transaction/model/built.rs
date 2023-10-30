use core::fmt;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use super::{Bytes, TransactionStatus, TxHash};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct BuiltTransaction {
    version: u8,
    created_at: DateTime<Utc>,
    status: TransactionStatus,
    tx_hash: TxHash,
    tx_body: Bytes,
    signatures: Option<HashMap<PublicKey, Signature>>,
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct Bytes64([u8; 64]);

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

type PublicKey = Bytes64;
type Signature = Bytes64;

#[cfg(test)]
mod tests {
    use chrono::DateTime;

    use crate::transaction::model::Hash32;

    use super::*;

    #[test]
    fn json_roundtrip() {
        let tx = BuiltTransaction {
            version: 3,
            created_at: DateTime::from_timestamp(0, 0).unwrap(),
            status: TransactionStatus::Built,
            tx_hash: Hash32([0; 32]),
            tx_body: Bytes([6; 100].to_vec()),
            signatures: Some(
                vec![(Bytes64([20; 64]), Bytes64([9; 64]))]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            ),
        };

        let serialised_tx = serde_json::to_string(&tx).unwrap();

        let deserialised_tx: BuiltTransaction = serde_json::from_str(&serialised_tx).unwrap();

        assert_eq!(tx, deserialised_tx)
    }
}
