use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
pub struct Bytes64(pub [u8; 64]);

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
