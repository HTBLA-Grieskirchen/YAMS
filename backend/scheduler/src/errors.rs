use error_stack::Report;
use molting::MigrationError;

#[derive(Debug, thiserror::Error)]
pub enum YamsSchedulerStartError {
    #[error("invalid scheduler timezone")]
    Timezone,
    #[error("scheduler store migration failed")]
    Migration,
    #[error("scheduler failed to start")]
    Start,
}

#[derive(Debug, thiserror::Error)]
pub enum SQLiteStateStoreError {
    #[error("sqlite persistence error")]
    Persistence,
    #[error("scheduler migration error")]
    Migration,
    #[error("scheduler store error")]
    Store,
}

pub fn libsql_error_to_store_error(_: libsql::Error) -> SQLiteStateStoreError {
    SQLiteStateStoreError::Store
}

pub fn migration_error_to_store_error<E: std::error::Error + Send + Sync + 'static>(
    error: MigrationError<E>,
) -> Report<SQLiteStateStoreError> {
    Report::new(error).change_context(SQLiteStateStoreError::Migration)
}
