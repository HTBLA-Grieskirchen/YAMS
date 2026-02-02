pub struct Migration;

use async_trait::async_trait;
use molting::UpMigration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        1
    }

    fn description(&self) -> Option<&'static str> {
        Some("Initial migration (animals, clients)")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                "
            CREATE TABLE IF NOT EXISTS animals (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                birthdate TEXT NOT NULL,
                animal_species TEXT NOT NULL,
                description TEXT NOT NULL,
                client_id TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (client_id) REFERENCES client(id)
            );

            CREATE TABLE IF NOT EXISTS clients (
                id TEXT PRIMARY KEY,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                birthdate TEXT NOT NULL,
                email TEXT NOT NULL,
                mobile_number TEXT NOT NULL,
                customer_number INTEGER NOT NULL UNIQUE,
                consent BOOLEAN NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );
        ",
            )
            .await?;
        Ok(())
    }
}
