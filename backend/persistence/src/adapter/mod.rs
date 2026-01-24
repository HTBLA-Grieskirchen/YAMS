use std::path::Path;

use crate::Error;
use crate::migration::run_migrations;
use libsql::Builder;

pub mod address;
pub mod animal;
pub mod client;
pub mod event;

pub struct SqliteAdapter {
    pub(crate) db: libsql::Connection,
}

impl SqliteAdapter {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                Error::Database(format!("Failed to create parent directories: {}", e))
            })?;
        }
        let db = Builder::new_local(path)
            .build()
            .await
            .map_err(|e| Error::Database(e.to_string()))?
            .connect()
            .map_err(|e| Error::Database(e.to_string()))?;

        let adapter = Self { db };
        adapter.run_migrations().await?;

        Ok(adapter)
    }

    async fn run_migrations(&self) -> Result<(), Error> {
        run_migrations(&self.db)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }
}
