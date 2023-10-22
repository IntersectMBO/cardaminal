//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "utxo")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub tx_hash: Vec<u8>,
    pub txo_index: i32,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub payment_cred: Vec<u8>,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub full_address: Vec<u8>,
    pub slot: i64,
    pub era: u16,
    #[sea_orm(column_type = "Binary(BlobSize::Blob(None))")]
    pub cbor: Vec<u8>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
