pub struct Migration;

use async_trait::async_trait;
use molting::UpMigration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        4
    }

    fn description(&self) -> Option<&'static str> {
        Some("UTF-8 Spaltennamen (stückzahl, straße_und_hausnummer)")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        rename_column_if_exists(
            transaction,
            "rechnungspositionen",
            "stueckzahl",
            "stückzahl",
        )
        .await?;
        rename_column_if_exists(
            transaction,
            "klienten",
            "strasse_und_hausnummer",
            "straße_und_hausnummer",
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

async fn rename_column_if_exists(
    transaction: &mut libsql::Transaction,
    table: &str,
    from: &str,
    to: &str,
) -> Result<(), libsql::Error> {
    if column_exists(transaction, table, from).await? {
        let quoted_to = format!("\"{}\"", to.replace('"', "\"\""));
        transaction
            .execute_batch(&format!(
                "ALTER TABLE {table} RENAME COLUMN {from} TO {quoted_to};"
            ))
            .await?;
    }

    Ok(())
}
