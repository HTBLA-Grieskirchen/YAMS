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

    pub async fn in_memory() -> Result<Self, Error> {
        let db = Builder::new_local(":memory:")
            .build()
            .await
            .map_err(|e| Error::Database(e.to_string()))?
            .connect()
            .map_err(|e| Error::Database(e.to_string()))?;

        let adapter = Self { db };
        adapter.run_migrations().await?;

        println!("In memory database created");
        let mut rows = adapter
            .db
            .query("SELECT name FROM sqlite_master WHERE type='table'", ())
            .await
            .unwrap();
        println!("Available tables:");
        while let Some(row) = rows.next().await.unwrap() {
            let table_name: String = row.get(0).unwrap();
            println!("  - {}", table_name);
        }

        Ok(adapter)
    }

    async fn run_migrations(&self) -> Result<(), Error> {
        run_migrations(&self.db)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        Ok(())
    }
}
