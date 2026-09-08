use async_trait::async_trait;
use molting::UpMigration;

pub struct Migration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        1
    }

    fn description(&self) -> Option<&'static str> {
        Some("Scheduler job state table")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                "
            CREATE TABLE IF NOT EXISTS _scheduler_job_state (
                job_id TEXT PRIMARY KEY NOT NULL,
                trigger_count INTEGER NOT NULL DEFAULT 0,
                last_run_at TEXT,
                last_success_at TEXT,
                next_run_at TEXT,
                last_error TEXT
            );
            ",
            )
            .await?;
        Ok(())
    }
}
