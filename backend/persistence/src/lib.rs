mod errors;
mod migrations;
pub mod repos;
mod uow;

use std::{path::Path, sync::Arc};

use async_lock::Mutex;
use migrations::MIGRATIONS;
pub use uow::*;
use yams_core::service::errors::PersistenceError;

use crate::errors::ToPersistenceResultExt;

pub struct SQLiteInstance {
    pub(crate) connection: Arc<Mutex<libsql::Connection>>,
}

impl SQLiteInstance {
    pub async fn local(path: impl AsRef<Path>) -> Result<Self, PersistenceError> {
        let path = path.as_ref();
        let connection = libsql::Builder::new_local(path)
            .build()
            .await
            .to_persistence()?
            .connect()
            .to_persistence()?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub async fn in_memory() -> Result<Self, PersistenceError> {
        let connection = libsql::Builder::new_local(":memory:")
            .build()
            .await
            .to_persistence()?
            .connect()
            .to_persistence()?;
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }

    pub async fn migrate_to_latest(&mut self) -> Result<(), PersistenceError> {
        MIGRATIONS.apply(self, None).await.to_persistence()
    }
}
