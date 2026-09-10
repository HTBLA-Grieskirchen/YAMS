use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use molting::{AppliableMigration, MigrationError, MigrationTarget};
use scheduler::{JobState, StateStore, StoreErrorKind};
use thiserror::Error;
use yams_sqlite::SQLiteInstance;

use crate::migrations::MIGRATIONS;

#[derive(Debug, Error)]
pub enum SQLiteStateStoreError {
    #[error("sqlite persistence error")]
    Persistence,
    #[error("scheduler store error")]
    Store,
}

fn libsql_error_to_store_error(_: libsql::Error) -> SQLiteStateStoreError {
    SQLiteStateStoreError::Store
}

pub struct SQLiteStateStore {
    instance: Arc<SQLiteInstance>,
}

impl SQLiteStateStore {
    pub fn new(instance: Arc<SQLiteInstance>) -> Self {
        Self { instance }
    }

    pub async fn migrate_to_latest(&mut self) -> Result<(), MigrationError<libsql::Error>> {
        MIGRATIONS.apply(self, None).await
    }
}

#[async_trait]
impl MigrationTarget<libsql::Transaction, libsql::Error> for SQLiteStateStore {
    fn get_current_version(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<Option<usize>, libsql::Error>> + '_>> {
        Box::pin(async move {
            let connection = self.instance.create_connection().await.map_err(|_| {
                libsql::Error::Misuse("scheduler migration connection failed".into())
            })?;
            let tx = connection
                .transaction_with_behavior(libsql::TransactionBehavior::Exclusive)
                .await?;
            tx.execute(
                "CREATE TABLE IF NOT EXISTS _scheduler_migration_history (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    version INTEGER,
                    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                )",
                (),
            )
            .await?;

            let mut rows = tx
                .query(
                    "SELECT version FROM _scheduler_migration_history ORDER BY applied_at DESC, version DESC LIMIT 1",
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
        let connection =
            self.instance.create_connection().await.map_err(|_| {
                libsql::Error::Misuse("scheduler migration connection failed".into())
            })?;
        let mut tx = connection
            .transaction_with_behavior(libsql::TransactionBehavior::Exclusive)
            .await?;

        implementation.run(&mut tx).await?;

        tx.execute(
            "INSERT INTO _scheduler_migration_history (version) VALUES (?1)",
            [new_version.map(|v| v as i64)],
        )
        .await?;

        tx.commit().await?;
        Ok(())
    }
}

impl StateStore for SQLiteStateStore {
    type Error = SQLiteStateStoreError;

    async fn load(&self, job_id: &str) -> Result<Option<JobState>, Self::Error> {
        let connection = self
            .instance
            .create_connection()
            .await
            .map_err(|_| SQLiteStateStoreError::Persistence)?;

        let mut rows = connection
            .query(
                "SELECT trigger_count, last_run_at, last_success_at, next_run_at, last_error
                 FROM _scheduler_job_state WHERE job_id = ?1",
                [job_id],
            )
            .await
            .map_err(libsql_error_to_store_error)?;

        let Some(row) = rows.next().await.map_err(libsql_error_to_store_error)? else {
            return Ok(None);
        };

        Ok(Some(JobState {
            job_id: job_id.to_string(),
            trigger_count: row.get::<u64>(0).map_err(libsql_error_to_store_error)? as u32,
            last_run_at: parse_optional_timestamp(row.get(1).map_err(libsql_error_to_store_error)?),
            last_success_at: parse_optional_timestamp(
                row.get(2).map_err(libsql_error_to_store_error)?,
            ),
            next_run_at: parse_optional_timestamp(row.get(3).map_err(libsql_error_to_store_error)?),
            last_error: row.get(4).map_err(libsql_error_to_store_error)?,
        }))
    }

    async fn save(&self, state: &JobState) -> Result<(), Self::Error> {
        let connection = self
            .instance
            .create_connection()
            .await
            .map_err(|_| SQLiteStateStoreError::Persistence)?;

        connection
            .execute(
                "INSERT INTO _scheduler_job_state (
                    job_id, trigger_count, last_run_at, last_success_at, next_run_at, last_error
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                ON CONFLICT(job_id) DO UPDATE SET
                    trigger_count = excluded.trigger_count,
                    last_run_at = excluded.last_run_at,
                    last_success_at = excluded.last_success_at,
                    next_run_at = excluded.next_run_at,
                    last_error = excluded.last_error",
                (
                    state.job_id.as_str(),
                    state.trigger_count as i64,
                    format_optional_timestamp(state.last_run_at),
                    format_optional_timestamp(state.last_success_at),
                    format_optional_timestamp(state.next_run_at),
                    state.last_error.as_deref(),
                ),
            )
            .await
            .map_err(libsql_error_to_store_error)?;
        Ok(())
    }

    async fn delete(&self, job_id: &str) -> Result<(), Self::Error> {
        let connection = self
            .instance
            .create_connection()
            .await
            .map_err(|_| SQLiteStateStoreError::Persistence)?;
        connection
            .execute(
                "DELETE FROM _scheduler_job_state WHERE job_id = ?1",
                [job_id],
            )
            .await
            .map_err(libsql_error_to_store_error)?;
        Ok(())
    }

    fn classify_error(error: &Self::Error) -> scheduler::StoreErrorKind
    where
        Self: Sized,
    {
        match error {
            SQLiteStateStoreError::Persistence => StoreErrorKind::Connection,
            SQLiteStateStoreError::Store => StoreErrorKind::Data,
        }
    }
}

fn parse_optional_timestamp(value: Option<String>) -> Option<DateTime<Utc>> {
    value.and_then(|raw| {
        DateTime::parse_from_rfc3339(&raw)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    })
}

fn format_optional_timestamp(value: Option<DateTime<Utc>>) -> Option<String> {
    value.map(|dt| dt.to_rfc3339())
}
