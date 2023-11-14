use pallas::crypto::key::ed25519;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Bytes, Bytes32, TxHash};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct BuiltTransaction {
    pub version: String,
    pub tx_hash: TxHash,
    pub tx_bytes: Bytes,
    pub signatures: Option<HashMap<PublicKey, Signature>>,
}

impl BuiltTransaction {
    pub fn sign(&self, secret_key: ed25519::SecretKey) -> ed25519::Signature {
        secret_key.sign(self.tx_hash.0)
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct Bytes64(pub [u8; 64]);

type PublicKey = Bytes32;
type Signature = Bytes64;

#[cfg(test)]
mod tests {
    use crate::transaction::model::Bytes32;

    use super::*;

    #[test]
    fn json_roundtrip() {
        let tx = BuiltTransaction {
            version: "3".into(),
            tx_hash: Bytes32([0; 32]),
            tx_bytes: Bytes([6; 100].to_vec()),
            signatures: Some(
                vec![(Bytes32([20; 32]), Bytes64([9; 64]))]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            ),
        };

        let serialised_tx = serde_json::to_string(&tx).unwrap();

        let deserialised_tx: BuiltTransaction = serde_json::from_str(&serialised_tx).unwrap();

        assert_eq!(tx, deserialised_tx)
    }
}
