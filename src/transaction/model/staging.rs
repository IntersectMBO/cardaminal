use pallas::{
    ledger::{
        addresses::Address as PallasAddress,
        primitives::{
            babbage::{
                ExUnits as PallasExUnits, NativeScript, PlutusData, PlutusV1Script, PlutusV2Script,
                TransactionInput,
            },
            Fragment,
        },
        traverse::wellknown::GenesisValues,
    },
    txbuilder::{
        self as Txb,
        plutus_script::RedeemerPurpose as TxbRedeemerPurpose,
        prelude::{MultiAsset, TransactionBuilder},
        transaction::Transaction,
        NetworkParams,
    },
};

use pallas::txbuilder::transaction as txb;

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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
    script_data_hash: Option<Hash32>,
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

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
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
    mem: u32,
    steps: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
struct Redeemers(HashMap<RedeemerPurpose, (PlutusData, Option<ExUnits>)>);

#[derive(PartialEq, Eq, Debug)]
pub struct Address(pub PallasAddress);

#[derive(Debug)]
pub enum Error {
    MalformedScript,
    MalformedDatum,
    ValidationError(Txb::ValidationError),
}

impl StagingTransaction {
    pub fn build(self, params: GenesisValues) -> Result<Transaction, Error> {
        let mut builder = TransactionBuilder::new(NetworkParams {
            genesis_values: params,
        });

        for input in self.inputs.iter() {
            let txin = txb::Input::build(input.tx_hash.0, input.tx_index as u64);

            builder = builder.input(txin, None);
        }

        for input in self.reference_inputs.unwrap_or_default() {
            let txin = txb::Input::build(input.tx_hash.0, input.tx_index as u64);

            builder = builder.input(txin, None);
        }

        for output in self.outputs.unwrap_or_default() {
            let txo = if let Some(assets) = output.assets {
                let txb_assets = assets
                    .0
                    .into_iter()
                    .map(|(pid, assets)| {
                        (
                            pid.0.into(),
                            assets
                                .into_iter()
                                .map(|(n, x)| (n.into(), x))
                                .collect::<HashMap<pallas::codec::utils::Bytes, _>>(),
                        )
                    })
                    .collect();

                txb::Output::multiasset(
                    output.address.0.to_vec(),
                    output.lovelace,
                    MultiAsset::<u64>::from_map(txb_assets),
                )
                .build()
            } else {
                txb::Output::lovelaces(output.address.0.to_vec(), output.lovelace).build()
            };

            builder = builder.output(txo);
        }

        if let Some(fee) = self.fee {
            builder = builder.fee(fee)
        }

        if let Some(massets) = self.mint {
            let txb_massets = massets
                .0
                .into_iter()
                .map(|(pid, assets)| {
                    (
                        pid.0.into(),
                        assets
                            .into_iter()
                            .map(|(n, x)| (n.into(), x))
                            .collect::<HashMap<pallas::codec::utils::Bytes, _>>(),
                    )
                })
                .collect();

            builder = builder.mint(MultiAsset::<i64>::from_map(txb_massets));
        }

        if let Some(x) = self.valid_from_slot {
            builder = builder.valid_from_slot(x)
        }

        if let Some(x) = self.invalid_from_slot {
            builder = builder.invalid_from_slot(x)
        }

        if let Some(nid) = self.network_id {
            builder = builder.network_id(nid)
        }

        for input in self.collateral_inputs.unwrap_or_default() {
            let txin = txb::Input::build(input.tx_hash.0, input.tx_index as u64);

            builder = builder.collateral(txin, None);
        }

        if let Some(coll_output) = self.collateral_output {
            builder = builder.collateral_return(
                txb::Output::lovelaces(coll_output.address.0.to_vec(), coll_output.lovelace)
                    .build(),
            );
        }

        for signer in self.disclosed_signers.unwrap_or_default() {
            builder = builder.require_signer(signer.0.into())
        }

        for script in self.scripts.unwrap_or_default() {
            match script.kind {
                ScriptKind::Native => {
                    let script = NativeScript::decode_fragment(&script.bytes.0)
                        .map_err(|_| Error::MalformedScript)?;

                    builder = builder.native_script(script);
                }
                ScriptKind::PlutusV1 => {
                    let script = PlutusV1Script(script.bytes.into());

                    builder = builder.plutus_v1_script(script);
                }
                ScriptKind::PlutusV2 => {
                    let script = PlutusV2Script(script.bytes.into());

                    builder = builder.plutus_v2_script(script);
                }
            }
        }

        for datum in self.datums.unwrap_or_default() {
            let pd = PlutusData::decode_fragment(&datum.0).map_err(|_| Error::MalformedDatum)?;

            builder = builder.plutus_data(pd)
        }

        if let Some(redeemers) = self.redeemers {
            for (redeemer, (pd, ex_units)) in redeemers.0.into_iter() {
                let ex_units = if let Some(ExUnits { mem, steps }) = ex_units {
                    PallasExUnits { mem, steps }
                } else {
                    todo!("ExUnits budget calculation not yet implement") // TODO
                };

                match redeemer {
                    RedeemerPurpose::Spend(txh, idx) => {
                        let rp = TxbRedeemerPurpose::Spend(TransactionInput {
                            transaction_id: txh.0.into(),
                            index: idx as u64,
                        });

                        builder = builder.redeemer(rp, pd, ex_units)
                    }
                    RedeemerPurpose::Mint(pid) => {
                        let rp = TxbRedeemerPurpose::Mint(pid.0.into());

                        builder = builder.redeemer(rp, pd, ex_units)
                    }
                    // RedeemerPurpose:: TODO
                    // RedeemerPurpose:: TODO
                };
            }
        };

        if let Some(h) = self.script_data_hash {
            builder = builder.script_data_hash(h.0.into())
        };

        // signature_amount_override: Option<u8>, // TODO
        // change_address: Option<Address>, // TODO

        builder.build().map_err(Error::ValidationError)
    }
}

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
                (RedeemerPurpose::Spend(Hash32([4; 32]), 0), (PlutusData::Array(vec![]), Some(ExUnits { mem: 1337, steps: 7331 }))),
                (RedeemerPurpose::Mint(Hash28([5; 28])), (PlutusData::Array(vec![]), None)),
            ].into_iter().collect::<HashMap<_, _>>())),
            signature_amount_override: Some(5),
            change_address: Some(Address(PallasAddress::from_str("addr1g9ekml92qyvzrjmawxkh64r2w5xr6mg9ngfmxh2khsmdrcudevsft64mf887333adamant").unwrap())),
            script_data_hash: Some(Hash32([0; 32])),
        };

        let serialised_tx = serde_json::to_string(&tx).unwrap();

        let deserialised_tx: StagingTransaction = serde_json::from_str(&serialised_tx).unwrap();

        assert_eq!(tx, deserialised_tx)
    }
}
