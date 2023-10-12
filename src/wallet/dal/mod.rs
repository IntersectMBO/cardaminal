pub mod migration; // TODO

use std::path::PathBuf;

use sea_orm::entity::prelude::*;
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

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
}
