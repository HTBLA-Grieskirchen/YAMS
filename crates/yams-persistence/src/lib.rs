mod errors;
mod migrations;
pub mod repos;
mod uow;

use std::{
    ops::Deref,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use async_lock::{Mutex, MutexGuardArc};
use error_stack::ResultExt;
use migrations::MIGRATIONS;
use tempdir::TempDir;
pub use uow::*;
use uuid::Uuid;
use yams_core::ports::RepositoryError;
use yams_core::{ErrorReportExt, ResultReport};

use crate::errors::{libsql_error_to_persistence_error, migration_error_to_persistence_error};

pub struct SQLiteInstance {
    variant: InstanceType,
    /// File-level pragmas (WAL, auto_vacuum) must run once; re-running
    /// `journal_mode=WAL` on every connection races close/checkpoint locks.
    database_configured: AtomicBool,
}

enum InstanceType {
    Local(libsql::Database),
    InMemory {
        // Needed to create multiple connections to the same memory db
        // see https://github.com/tursodatabase/libsql/issues/1376
        connection_lock: Arc<Mutex<()>>,
        connection: Arc<libsql::Connection>,
    },
    TempDir {
        temp_dir: Arc<TempDir>,
        db: libsql::Database,
    },
}

pub enum SQLiteConnection {
    Local(libsql::Connection),
    InMemory {
        connection_lock: MutexGuardArc<()>,
        connection: Arc<libsql::Connection>,
    },
    TempDir {
        temp_dir: Arc<TempDir>,
        connection: libsql::Connection,
    },
}

impl Deref for SQLiteConnection {
    type Target = libsql::Connection;

    fn deref(&self) -> &Self::Target {
        match self {
            SQLiteConnection::Local(connection) => connection,
            SQLiteConnection::InMemory { connection, .. } => connection,
            SQLiteConnection::TempDir { connection, .. } => connection,
        }
    }
}

impl SQLiteInstance {
    pub async fn local(path: impl AsRef<Path>) -> ResultReport<Self, RepositoryError> {
        let path = path.as_ref();
        let this = Self {
            variant: InstanceType::Local(
                libsql::Builder::new_local(path)
                    .build()
                    .await
                    .contextualize_with(libsql_error_to_persistence_error)?,
            ),
            database_configured: AtomicBool::new(false),
        };
        // Sanity check connection (also configures WAL once)
        drop(this.create_connection().await?);
        Ok(this)
    }

    pub async fn in_temp_dir() -> ResultReport<Self, RepositoryError> {
        let temp_dir =
            TempDir::new("yams-persistence").contextualize(RepositoryError::Connection)?;
        let path = temp_dir.path().join("yams.db");
        Ok(Self {
            variant: InstanceType::TempDir {
                db: libsql::Builder::new_local(path)
                    .build()
                    .await
                    .contextualize_with(libsql_error_to_persistence_error)?,
                temp_dir: Arc::new(temp_dir),
            },
            database_configured: AtomicBool::new(false),
        })
    }

    pub async fn in_memory() -> ResultReport<Self, RepositoryError> {
        let memory_uuid = Uuid::new_v4();
        let db =
            libsql::Builder::new_local(format!("file:{}.db?mode=memory&cache=shared", memory_uuid))
                .build()
                .await
                .contextualize_with(libsql_error_to_persistence_error)?;
        let connection = db
            .connect()
            .contextualize_with(libsql_error_to_persistence_error)?;
        Self::configure_database(&connection).await?;
        let connection = Self::initialize_connection(connection).await?;
        Ok(Self {
            variant: InstanceType::InMemory {
                connection: Arc::new(connection),
                connection_lock: Arc::new(Mutex::new(())),
            },
            database_configured: AtomicBool::new(true),
        })
    }

    pub async fn migrate_to_latest(&mut self) -> ResultReport<(), RepositoryError> {
        let mut connection = self.create_connection().await?;
        MIGRATIONS
            .apply(&mut connection, None)
            .await
            .contextualize_with(migration_error_to_persistence_error)
    }

    pub(crate) async fn create_connection(
        &self,
    ) -> ResultReport<SQLiteConnection, RepositoryError> {
        match &self.variant {
            InstanceType::Local(db) => {
                let connection = db
                    .connect()
                    .contextualize_with(libsql_error_to_persistence_error)?;
                self.ensure_database_configured(&connection).await?;
                let connection = Self::initialize_connection(connection).await?;
                Ok(SQLiteConnection::Local(connection))
            }
            InstanceType::InMemory {
                connection,
                connection_lock,
            } => {
                let guard = connection_lock.lock_arc().await;
                Ok(SQLiteConnection::InMemory {
                    connection: Arc::clone(connection),
                    connection_lock: guard,
                })
            }
            InstanceType::TempDir { db, temp_dir } => {
                let connection = db
                    .connect()
                    .contextualize_with(libsql_error_to_persistence_error)?;
                self.ensure_database_configured(&connection).await?;
                let connection = Self::initialize_connection(connection).await?;
                Ok(SQLiteConnection::TempDir {
                    connection,
                    temp_dir: Arc::clone(temp_dir),
                })
            }
        }
    }

    async fn ensure_database_configured(
        &self,
        connection: &libsql::Connection,
    ) -> ResultReport<(), RepositoryError> {
        if self.database_configured.load(Ordering::Acquire) {
            return Ok(());
        }
        Self::configure_database(connection).await?;
        self.database_configured.store(true, Ordering::Release);
        Ok(())
    }

    /// File-level settings. Must run with busy_timeout first; only once per DB.
    async fn configure_database(
        connection: &libsql::Connection,
    ) -> ResultReport<(), RepositoryError> {
        let statements = [
            "PRAGMA busy_timeout=5000",
            "PRAGMA journal_mode=WAL",
            "PRAGMA auto_vacuum=INCREMENTAL",
        ];
        for statement in statements {
            connection
                .query(statement, ())
                .await
                .contextualize_with(libsql_error_to_persistence_error)
                .attach(format!("while configuring database with stmt: {statement}"))?;
        }
        Ok(())
    }

    /// Per-connection settings. busy_timeout first so later work can wait on locks.
    async fn initialize_connection(
        connection: libsql::Connection,
    ) -> ResultReport<libsql::Connection, RepositoryError> {
        let statements = [
            "PRAGMA busy_timeout=5000",
            "PRAGMA synchronous=NORMAL",
            "PRAGMA foreign_keys=ON",
            "PRAGMA cache_size=-64000",
            "PRAGMA temp_store=MEMORY",
            "PRAGMA mmap_size=2147483648",
        ];
        for statement in statements {
            connection
                .query(statement, ())
                .await
                .contextualize_with(libsql_error_to_persistence_error)
                .attach(format!(
                    "while initializing connection with stmt: {statement}"
                ))?;
        }
        Ok(connection)
    }
}
