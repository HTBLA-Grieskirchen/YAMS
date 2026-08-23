pub struct Migration;

use async_trait::async_trait;
use molting::UpMigration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        4
    }

    fn description(&self) -> Option<&'static str> {
        Some("UTF-8 Spaltenname stückzahl")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                "ALTER TABLE rechnungspositionen RENAME COLUMN stueckzahl TO stückzahl;",
            )
            .await?;

        Ok(())
    }
}
