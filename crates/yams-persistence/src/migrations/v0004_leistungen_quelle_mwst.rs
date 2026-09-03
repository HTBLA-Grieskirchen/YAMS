pub struct Migration;

use async_trait::async_trait;
use molting::UpMigration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        4
    }

    fn description(&self) -> Option<&'static str> {
        Some("Add leistungen.quelle_mwst for databases created before the column landed in v0002")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        if column_exists(transaction, "leistungen", "quelle_mwst").await? {
            return Ok(());
        }

        transaction
            .execute(
                "ALTER TABLE leistungen ADD COLUMN quelle_mwst TEXT NOT NULL DEFAULT '0'",
                (),
            )
            .await?;
        Ok(())
    }
}

async fn column_exists(
    transaction: &mut libsql::Transaction,
    table: &str,
    column: &str,
) -> Result<bool, libsql::Error> {
    let mut rows = transaction
        .query(&format!("PRAGMA table_info({table})"), ())
        .await?;
    while let Some(row) = rows.next().await? {
        let name: String = row.get(1)?;
        if name == column {
            return Ok(true);
        }
    }
    Ok(false)
}
