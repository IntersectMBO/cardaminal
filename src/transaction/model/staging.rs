use pallas::ledger::addresses::Address as PallasAddress;
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Bytes, Hash28, TransactionStatus, TxHash};

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
pub struct OutputAssets(pub HashMap<PolicyId, HashMap<AssetName, u64>>);

#[derive(PartialEq, Eq, Debug)]
pub struct MintAssets(pub HashMap<PolicyId, HashMap<AssetName, i64>>);

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
pub enum RedeemerPurpose {
    Spend(TxHash, usize),
    Mint(PolicyId),
    // Reward TODO
    // Cert TODO
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct ExUnits {
    mem: u64,
    steps: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Redeemers(HashMap<RedeemerPurpose, Option<ExUnits>>);

#[derive(PartialEq, Eq, Debug)]
pub struct Address(pub PallasAddress);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use chrono::DateTime;
    use pallas::ledger::addresses::Address as PallasAddress;

    use crate::transaction::model::Hash32;

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
