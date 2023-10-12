pub mod entities;
pub mod migration; // TODO

use std::path::PathBuf;

use pallas::ledger::addresses::{Address, ShelleyPaymentPart};
use pallas::ledger::traverse::{MultiEraInput, MultiEraOutput};
use sea_orm::entity::prelude::*;
use sea_orm::{Condition, Database, TransactionTrait};
use sea_orm_migration::MigratorTrait;

use self::entities::prelude::Utxo;
use self::entities::utxo;
use self::migration::Migrator;

pub struct WalletDB {
    pub name: String,
    pub path: PathBuf,
    pub conn: DatabaseConnection,
}

impl WalletDB {
    pub async fn open(name: &String, path: PathBuf) -> Result<Self, DbErr> {
        let sqlite_url = format!("sqlite:{}/state.sqlite?mode=rwc", path.display()); // TODO
        let db = Database::connect(sqlite_url).await?;
        Ok(Self {
            name: name.clone(),
            path,
            conn: db,
        })
    }

    pub async fn migrate_up(&self) -> Result<(), DbErr> {
        Migrator::up(&self.conn, None).await
    }

    pub async fn insert_utxos(
        &self,
        utxos: Vec<(MultiEraInput<'_>, MultiEraOutput<'_>, u64)>,
    ) -> Result<(), DbErr> {
        let txn = self.conn.begin().await?;

        for (txin, txout, slot) in utxos {
            let (tx_hash, txo_index) = (txin.hash(), txin.index());

            let address = txout.address().unwrap();

            let address_bytes = address.to_vec();

            let payment_cred = match address {
                Address::Shelley(s) => match s.payment() {
                    ShelleyPaymentPart::Key(h) => *h,
                    ShelleyPaymentPart::Script(_) => {
                        unimplemented!("cannot store script controlled utxos")
                    }
                },
                _ => unimplemented!("cannot store byron address controlled utxos"),
            };

            let era = match txout {
                MultiEraOutput::Byron(_) => 0,
                MultiEraOutput::AlonzoCompatible(_) => 1,
                MultiEraOutput::Babbage(_) => 2,
                _ => unreachable!("unexpected txout era"),
            };

            let utxo_model = entities::utxo::ActiveModel {
                tx_hash: sea_orm::ActiveValue::Set(tx_hash.to_vec()),
                txo_index: sea_orm::ActiveValue::Set(txo_index as i32),
                payment_cred: sea_orm::ActiveValue::Set(payment_cred.to_vec()),
                full_address: sea_orm::ActiveValue::Set(address_bytes),
                slot: sea_orm::ActiveValue::Set(slot as i64),
                era: sea_orm::ActiveValue::Set(era),
                cbor: sea_orm::ActiveValue::Set(txout.encode()),
                ..Default::default()
            };

            let _ = Utxo::insert(utxo_model).exec(&txn).await?;
        }

        txn.commit().await
    }

    pub async fn remove_utxos(&self, utxos: Vec<MultiEraInput<'_>>) -> Result<(), DbErr> {
        let txn = self.conn.begin().await?;

        for txin in utxos {
            if let Some(utxo_model) = Utxo::find()
                .filter(
                    Condition::all()
                        .add(utxo::Column::TxHash.eq(txin.hash().to_vec()))
                        .add(utxo::Column::TxoIndex.eq(txin.index())),
                )
                .one(&txn)
                .await?
            {
                let _ = utxo_model.delete(&txn).await?;
            }
        }

        txn.commit().await
    }

    pub async fn fetch_utxos_for_payment_cred(
        &self,
        cred: [u8; 28],
    ) -> Result<Vec<utxo::Model>, DbErr> {
        Utxo::find()
            .filter(utxo::Column::PaymentCred.eq(cred.to_vec()))
            .all(&self.conn)
            .await
    }

    // pub async fn fetch_utxos_for_address(&self, address: Address) -> () {
    //     ()
    // }

    // // TODO: balance type
    // pub async fn insert_history_tx(&self, tx_hash: [u8; 32], slot: u64, tx_block_index: u16, delta: Vec<u8> ) -> {
    //     ()
    // }

    // pub async fn fetch_history_for_payment_cred(&self, cred: [u8; 28]) -> () {
    //     ()
    // }

    // pub async fn insert_recent_point(&self, slot: u64, block_hash: [u8; 32]) -> () {
    //     ()
    // }

    // pub async fn remove_recent_points_after_slot(&self, slot: u64) -> () {
    //     ()
    // }

    // pub async fn remove_recent_points_before_slot(&self, slot: u64) -> () {
    //     ()
    // }
}
