use std::sync::LazyLock;

use molting::{MigrationRegistry, UpMigration};

mod v0001_scheduler_job_state;

type Registry = MigrationRegistry<dyn UpMigration<libsql::Transaction, libsql::Error>>;

pub static SCHEDULER_MIGRATIONS: LazyLock<Registry> = LazyLock::new(|| {
    let mut registry: Registry = MigrationRegistry::new();
    registry.add(v0001_scheduler_job_state::Migration);
    registry
});

pub const MIGRATION_HISTORY_TABLE: &str = "_scheduler_migration_history";
