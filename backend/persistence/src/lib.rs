mod errors;
mod migrations;
pub mod repos;
mod uow;

use std::{ops::Deref, path::Path, sync::Arc};

use async_lock::{Mutex, MutexGuardArc};
use error_stack::ResultExt;
use migrations::MIGRATIONS;
use tempdir::TempDir;
pub use uow::*;
use uuid::Uuid;
use yams_core::service::errors::{CoreResult, ErrorReportExt, PersistenceError};

use crate::errors::{libsql_error_to_persistence_error, migration_error_to_persistence_error};

pub struct SQLiteInstance {
    variant: InstanceType,
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
    pub async fn local(path: impl AsRef<Path>) -> CoreResult<Self, PersistenceError> {
        let path = path.as_ref();
        let this = Self {
            variant: InstanceType::Local(
                libsql::Builder::new_local(path)
                    .build()
                    .await
                    .contextualize_with(libsql_error_to_persistence_error)?,
            ),
        };
        // Sanity check connection
        drop(this.create_connection().await?);
        Ok(this)
    }

    pub async fn in_temp_dir() -> CoreResult<Self, PersistenceError> {
        let temp_dir =
            TempDir::new("yams-persistence").contextualize(PersistenceError::ConnectionError)?;
        let path = temp_dir.path().join("yams.db");
        Ok(Self {
            variant: InstanceType::TempDir {
                db: libsql::Builder::new_local(path)
                    .build()
                    .await
                    .contextualize_with(libsql_error_to_persistence_error)?,
                temp_dir: Arc::new(temp_dir),
            },
        })
    }

    pub async fn in_memory() -> CoreResult<Self, PersistenceError> {
        let memory_uuid = Uuid::new_v4();
        let db =
            libsql::Builder::new_local(format!("file:{}.db?mode=memory&cache=shared", memory_uuid))
                .build()
                .await
                .contextualize_with(libsql_error_to_persistence_error)?;
        Ok(Self {
            variant: InstanceType::InMemory {
                connection: Arc::new(
                    Self::initialize_connection(
                        db.connect()
                            .contextualize_with(libsql_error_to_persistence_error)?,
                    )
                    .await?,
                ),
                connection_lock: Arc::new(Mutex::new(())),
            },
        })
    }

    pub async fn migrate_to_latest(&mut self) -> CoreResult<(), PersistenceError> {
        let mut connection = self.create_connection().await?;
        MIGRATIONS
            .apply(&mut connection, None)
            .await
            .contextualize_with(migration_error_to_persistence_error)
    }

    pub(crate) async fn create_connection(&self) -> CoreResult<SQLiteConnection, PersistenceError> {
        match &self.variant {
            InstanceType::Local(db) => {
                let connection = db
                    .connect()
                    .contextualize_with(libsql_error_to_persistence_error)?;
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
                let connection = Self::initialize_connection(connection).await?;
                Ok(SQLiteConnection::TempDir {
                    connection,
                    temp_dir: Arc::clone(temp_dir),
                })
            }
        }
    }

    async fn initialize_connection(
        connection: libsql::Connection,
    ) -> CoreResult<libsql::Connection, PersistenceError> {
        let statements = vec![
            "PRAGMA journal_mode=WAL",
            "PRAGMA busy_timeout=5000",
            "PRAGMA synchronous=NORMAL",
            "PRAGMA foreign_keys=ON",
            "PRAGMA auto_vacuum=INCREMENTAL",
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
