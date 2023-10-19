pub mod entities;
pub mod migration;

use std::path::PathBuf;

use pallas::ledger::addresses::{Address, ShelleyPaymentPart};
use pallas::ledger::traverse::{MultiEraInput, MultiEraOutput};
use sea_orm::entity::prelude::*;
use sea_orm::{Condition, Database, Order, Paginator, QueryOrder, SelectModel, TransactionTrait};
use sea_orm_migration::MigratorTrait;

use self::entities::prelude::{ProtocolParameters, RecentPoints, TxHistory, Utxo};
use self::entities::{protocol_parameters, recent_points, tx_history, utxo};
use self::migration::Migrator;

static DEFAULT_PAGE_SIZE: u64 = 20;

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

    // UTxOs

    pub async fn insert_utxos(
        &self,
        utxos: Vec<([u8; 32], usize, MultiEraOutput<'_>, u64)>,
    ) -> Result<(), DbErr> {
        let txn = self.conn.begin().await?;

        for (tx_hash, txo_index, txout, slot) in utxos {
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

    pub async fn remove_utxos(
        &self,
        utxos: Vec<MultiEraInput<'_>>,
    ) -> Result<Vec<utxo::Model>, DbErr> {
        let txn = self.conn.begin().await?;

        let mut removed = vec![];

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
                removed.push(utxo_model.clone());

                let _ = utxo_model.delete(&txn).await?;
            }
        }

        txn.commit().await?;

        Ok(removed)
    }

    pub fn paginate_utxos(
        &self,
        order: Order,
        page_size: Option<u64>,
    ) -> Paginator<'_, DatabaseConnection, SelectModel<utxo::Model>> {
        Utxo::find()
            .order_by(tx_history::Column::Slot, order.clone())
            .paginate(&self.conn, page_size.unwrap_or(DEFAULT_PAGE_SIZE))
    }

    pub async fn paginate_utxos_for_address(
        &self,
        address: Address,
        order: Order,
        page_size: Option<u64>,
    ) -> Paginator<'_, DatabaseConnection, SelectModel<utxo::Model>> {
        Utxo::find()
            .filter(utxo::Column::FullAddress.eq(address.to_vec()))
            .order_by(tx_history::Column::Slot, order.clone())
            .paginate(&self.conn, page_size.unwrap_or(DEFAULT_PAGE_SIZE))
    }

    // Transaction History

    // TODO: balance type
    pub async fn insert_history_tx(
        &self,
        tx_hash: [u8; 32],
        slot: u64,
        tx_block_index: u16,
        delta: Vec<u8>,
    ) -> Result<(), DbErr> {
        let history_model = entities::tx_history::ActiveModel {
            tx_hash: sea_orm::ActiveValue::Set(tx_hash.to_vec()),
            slot: sea_orm::ActiveValue::Set(slot as i64),
            block_index: sea_orm::ActiveValue::Set(tx_block_index.into()),
            balance_delta: sea_orm::ActiveValue::Set(delta),
            ..Default::default()
        };

        let _ = TxHistory::insert(history_model).exec(&self.conn).await?;

        Ok(())
    }

    pub fn paginate_tx_history(
        &self,
        order: Order,
        page_size: Option<u64>,
    ) -> Paginator<'_, DatabaseConnection, SelectModel<tx_history::Model>> {
        TxHistory::find()
            .order_by(tx_history::Column::Slot, order.clone())
            .order_by(tx_history::Column::BlockIndex, order)
            .paginate(&self.conn, page_size.unwrap_or(DEFAULT_PAGE_SIZE))
    }

    // Recent Points

    pub async fn insert_recent_point(&self, slot: u64, block_hash: [u8; 32]) -> Result<(), DbErr> {
        let point_model = entities::recent_points::ActiveModel {
            slot: sea_orm::ActiveValue::Set(slot as i64),
            block_hash: sea_orm::ActiveValue::Set(block_hash.into()),
            ..Default::default()
        };

        let _ = RecentPoints::insert(point_model).exec(&self.conn).await?;

        Ok(())
    }

    /// Paginate entries in Recents Points table in descending order
    pub fn paginate_recent_points(
        &self,
        page_size: Option<u64>,
    ) -> Paginator<'_, DatabaseConnection, SelectModel<recent_points::Model>> {
        RecentPoints::find()
            .order_by_desc(recent_points::Column::Slot)
            .paginate(&self.conn, page_size.unwrap_or(DEFAULT_PAGE_SIZE))
    }

    pub async fn remove_recent_points_before_slot(&self, slot: u64) -> Result<(), DbErr> {
        let txn = self.conn.begin().await?;

        let point_models = RecentPoints::find()
            .filter(Condition::all().add(recent_points::Column::Slot.lt(slot)))
            .all(&txn)
            .await?;

        for point_model in point_models {
            let _ = point_model.delete(&txn).await?;
        }

        txn.commit().await
    }

    // Protocol Parameters

    pub async fn insert_protocol_parameters(
        &self,
        slot: u64,
        tx_block_index: u16,
        update_cbor: Vec<u8>,
    ) -> Result<(), DbErr> {
        let pparams_model = entities::protocol_parameters::ActiveModel {
            slot: sea_orm::ActiveValue::Set(slot as i64),
            block_index: sea_orm::ActiveValue::Set(tx_block_index.into()),
            update_cbor: sea_orm::ActiveValue::Set(update_cbor),
            ..Default::default()
        };

        let _ = ProtocolParameters::insert(pparams_model)
            .exec(&self.conn)
            .await?;

        Ok(())
    }

    /// Fetch the CBOR of the most recent protocol parameters seen on-chain
    pub async fn fetch_latest_protocol_parameters(&self) -> Result<Option<Vec<u8>>, DbErr> {
        let res = ProtocolParameters::find()
            .order_by_desc(protocol_parameters::Column::Slot)
            .order_by_desc(protocol_parameters::Column::BlockIndex)
            .one(&self.conn)
            .await?;

        Ok(res.map(|r| r.update_cbor))
    }

    // Rollback

    /// Remove all records from WalletDB created for slots after the specified slot
    pub async fn rollback_to_slot(&self, slot: u64) -> Result<(), DbErr> {
        let txn = self.conn.begin().await?;

        // UTxOs

        let point_models = RecentPoints::find()
            .filter(Condition::all().add(recent_points::Column::Slot.gt(slot)))
            .all(&txn)
            .await?;

        for point_model in point_models {
            let _ = point_model.delete(&txn).await?;
        }

        // Transaction History

        let tx_models = TxHistory::find()
            .filter(Condition::all().add(tx_history::Column::Slot.gt(slot)))
            .all(&txn)
            .await?;

        for tx_model in tx_models {
            let _ = tx_model.delete(&txn).await?;
        }

        // Recent Points

        let points_models = RecentPoints::find()
            .filter(Condition::all().add(recent_points::Column::Slot.gt(slot)))
            .all(&txn)
            .await?;

        for point_model in points_models {
            let _ = point_model.delete(&txn).await?;
        }

        // Protocol Parameters

        let pparams_models = ProtocolParameters::find()
            .filter(Condition::all().add(protocol_parameters::Column::Slot.gt(slot)))
            .all(&txn)
            .await?;

        for pparams_model in pparams_models {
            let _ = pparams_model.delete(&txn).await?;
        }

        Ok(())
    }
}
