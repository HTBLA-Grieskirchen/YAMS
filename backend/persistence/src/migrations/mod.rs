use std::pin::Pin;
use std::sync::LazyLock;

use async_trait::async_trait;
use molting::{AppliableMigration, MigrationRegistry, MigrationTarget, UpMigration};

use crate::SQLiteInstance;

mod v0001_initial;

type Registry = MigrationRegistry<dyn UpMigration<libsql::Transaction, libsql::Error>>;

pub static MIGRATIONS: LazyLock<Registry> = LazyLock::new(|| {
    let mut registry: Registry = MigrationRegistry::new();
    registry.add(v0001_initial::Migration);
    registry
});

#[async_trait]
impl MigrationTarget<libsql::Transaction, libsql::Error> for SQLiteInstance {
    fn get_current_version(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Option<usize>, libsql::Error>> + '_>> {
        let tx = self.connection.transaction_with_behavior(libsql::TransactionBehavior::Exclusive);
        Box::pin(async move {
            let tx = tx.await?;
            // Create migrations table if it doesn't exist
            tx.execute(
                "CREATE TABLE IF NOT EXISTS _migration_history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    version INTEGER,
                    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )",
                (),
            )
            .await?;

            // Get the latest version based on the most recent application time
            let mut rows = tx
                .query(
                    "SELECT version FROM _migration_history ORDER BY applied_at DESC LIMIT 1",
                    (),
                )
                .await?;

            tx.commit().await?;
            let Some(row) = rows.next().await? else {
                return Ok(None);
            };
            let version: Option<u64> = row.get(0)?;
            Ok(version.map(|v| v as usize))
        })
    }

    async fn apply_migration(
        &mut self,
        new_version: Option<usize>,
        implementation: impl AppliableMigration<libsql::Transaction, libsql::Error> + Send,
    ) -> Result<(), libsql::Error> {
        let mut tx = self.connection.transaction_with_behavior(libsql::TransactionBehavior::Exclusive).await?;

        implementation.run(&mut tx).await?;

        tx.execute(
            "INSERT INTO _migration_history (version) VALUES (?1)",
            [new_version.map(|v| v as i64)],
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
