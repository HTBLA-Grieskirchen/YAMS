use std::path::Path;
use libsql::Builder;
use libsql_migration::dir::migrate as migrate_dir;
use crate::Error;

pub mod address;
pub mod client;
pub mod animal;
pub mod event;

pub struct SqliteAdapter {
    pub(crate) db: libsql::Connection,
}

impl SqliteAdapter {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let db = Builder::new_local(url)
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
        let migrations_path = Path::new("backend/persistence/migrations");
        let full_path = std::env::current_dir().unwrap().join(migrations_path);
        
        // Ensure migrations path exists
        if !full_path.exists() {
            return Err(Error::Internal(format!("Migrations directory not found: {:?}", full_path)));
        }

        migrate_dir(&self.db, full_path)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        
        Ok(())
    }
}
