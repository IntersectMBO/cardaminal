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
        traverse::{wellknown::GenesisValues, ComputeHash},
    },
    txbuilder::{
        self as Txb,
        plutus_script::RedeemerPurpose as TxbRedeemerPurpose,
        prelude::{MultiAsset, TransactionBuilder},
        NetworkParams,
    },
};

use pallas::txbuilder::transaction as txb;

use std::{collections::HashMap, ops::Deref};

use serde::{Deserialize, Serialize};

use super::{built::BuiltTransaction, Bytes, Hash28, Hash32, TransactionStatus, TxHash};

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct StagingTransaction {
    version: String,
    pub status: TransactionStatus,
    pub inputs: Option<Vec<Input>>,
    pub reference_inputs: Option<Vec<Input>>,
    pub outputs: Option<Vec<Output>>,
    pub fee: Option<u64>,
    pub mint: Option<MintAssets>,
    pub valid_from_slot: Option<u64>,
    pub invalid_from_slot: Option<u64>,
    pub network_id: Option<u32>,
    pub collateral_inputs: Option<Vec<Input>>,
    pub collateral_output: Option<CollateralOutput>,
    pub disclosed_signers: Option<Vec<PubKeyHash>>,
    pub scripts: Option<Vec<Script>>,
    pub datums: Option<Vec<DatumBytes>>,
    pub redeemers: Option<Redeemers>,
    pub script_data_hash: Option<Hash32>,
    pub signature_amount_override: Option<u8>,
    pub change_address: Option<Address>,
}
impl StagingTransaction {
    pub fn new() -> Self {
        Self {
            version: String::from("v1"),
            status: TransactionStatus::Staging,
            ..Default::default()
        }
    }
}

pub type PubKeyHash = Hash28;
pub type ScriptHash = Hash28;
pub type ScriptBytes = Bytes;
pub type PolicyId = ScriptHash;
pub type DatumBytes = Bytes;
pub type AssetName = Bytes;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Input {
    tx_hash: TxHash,
    tx_index: usize,
}
impl Input {
    pub fn new(tx_hash: TxHash, tx_index: usize) -> Self {
        Self { tx_hash, tx_index }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Output {
    pub address: Address,
    pub lovelace: u64,
    pub assets: Option<OutputAssets>,
    pub datum: Option<Datum>,
    pub script: Option<Script>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct OutputAssets(pub HashMap<PolicyId, HashMap<AssetName, u64>>);

#[derive(PartialEq, Eq, Debug)]
pub struct MintAssets(pub HashMap<PolicyId, HashMap<AssetName, i64>>);

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct CollateralOutput {
    pub address: Address,
    pub lovelace: u64,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ScriptKind {
    Native,
    PlutusV1,
    PlutusV2,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Script {
    pub kind: ScriptKind,
    pub bytes: ScriptBytes,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DatumKind {
    Hash,
    Inline,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Datum {
    pub kind: DatumKind,
    pub bytes: DatumBytes,
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
pub struct Redeemers(HashMap<RedeemerPurpose, (PlutusData, Option<ExUnits>)>);

#[derive(PartialEq, Eq, Debug)]
pub struct Address(pub PallasAddress);

impl Deref for Address {
    type Target = PallasAddress;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PallasAddress> for Address {
    fn from(value: PallasAddress) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub enum Error {
    MalformedScript,
    MalformedDatum,
    ValidationError(Txb::ValidationError),
}

impl StagingTransaction {
    pub fn build(self, params: GenesisValues) -> Result<BuiltTransaction, Error> {
        let mut builder = TransactionBuilder::new(NetworkParams {
            genesis_values: params,
        });

        for input in self.inputs.unwrap_or_default() {
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
                    } // RedeemerPurpose:: TODO
                      // RedeemerPurpose:: TODO
                };
            }
        };

        if let Some(h) = self.script_data_hash {
            builder = builder.script_data_hash(h.0.into())
        };

        // signature_amount_override: Option<u8>, // TODO
        // change_address: Option<Address>, // TODO

        let pallas_tx = builder.build().map_err(Error::ValidationError)?;

        Ok(BuiltTransaction {
            version: self.version,
            tx_hash: Hash32(*pallas_tx.body.compute_hash()),
            tx_bytes: Bytes(pallas_tx.encode_fragment().unwrap()),
            signatures: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use pallas::ledger::addresses::Address as PallasAddress;

    use crate::transaction::model::Hash32;

    use super::*;

    #[test]
    fn json_roundtrip() {
        let tx = StagingTransaction {
            version: String::from("v1"),
            status: TransactionStatus::Staging,
            inputs: Some(
                vec![
                    Input {
                        tx_hash: Hash32([0; 32]),
                        tx_index: 1
                    }
                ]
            ) ,
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
        dbg!(&serialised_tx);

        let deserialised_tx: StagingTransaction = serde_json::from_str(&serialised_tx).unwrap();

        assert_eq!(tx, deserialised_tx)
    }
}
