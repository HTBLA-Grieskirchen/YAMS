use std::path::Path;

use crate::Error;
use crate::migrations::migrate_up;
use async_trait::async_trait;
use crate::adapter::libsql_uow::LibSqlUnitOfWorkProvider;
pub use crate::adapter::libsql_uow::LibSqlUnitOfWorkProvider as SqliteUnitOfWorkProvider;
use yams_core::service::ports::repos::{UnitOfWorkImpl, UnitOfWorkProvider};
use yams_core::service::errors::PersistenceError;
use libsql::Builder;

pub mod libsql_uow;

pub struct SqliteAdapter {
    provider: LibSqlUnitOfWorkProvider,
}

#[async_trait]
impl UnitOfWorkProvider for SqliteAdapter {
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkImpl>, PersistenceError> {
        self.provider.begin().await
    }
}

impl SqliteAdapter {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                Error::Unknown(anyhow::anyhow!("Failed to create parent directories: {}", e))
            })?;
        }
        let db = Builder::new_local(path)
            .build()
            .await
            .map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?
            .connect()
            .map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?;

        let adapter = Self { 
            provider: LibSqlUnitOfWorkProvider::new(db)
        };
        adapter.run_migrations().await?;

        Ok(adapter)
    }

    pub async fn in_memory() -> Result<Self, Error> {
        let db = Builder::new_local(":memory:")
            .build()
            .await
            .map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?
            .connect()
            .map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?;

        let adapter = Self { 
            provider: LibSqlUnitOfWorkProvider::new(db)
        };
        adapter.run_migrations().await?;

        Ok(adapter)
    }

    async fn run_migrations(&self) -> Result<(), Error> {
        migrate_up(self.provider.conn()).await?;
        Ok(())
    }
}
