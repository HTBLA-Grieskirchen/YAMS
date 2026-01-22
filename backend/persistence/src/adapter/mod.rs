use libsql::Builder;
use crate::Error;
use crate::migration::run_migrations;

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
        run_migrations(&self.db)
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        
        Ok(())
    }
}
