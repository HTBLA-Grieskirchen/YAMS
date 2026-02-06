mod errors;
mod migrations;
pub mod repos;
mod uow;

use std::{ops::Deref, path::Path, sync::Arc};

use async_lock::{Mutex, MutexGuardArc};
use migrations::MIGRATIONS;
use tempdir::TempDir;
pub use uow::*;
use uuid::Uuid;
use yams_core::service::errors::PersistenceError;

use crate::errors::ToPersistenceResultExt;

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
    pub async fn local(path: impl AsRef<Path>) -> Result<Self, PersistenceError> {
        let path = path.as_ref();
        let this = Self {
            variant: InstanceType::Local(
                libsql::Builder::new_local(path)
                    .build()
                    .await
                    .to_persistence()?,
            ),
        };
        // Sanity check connection
        drop(this.create_connection().await?);
        Ok(this)
    }

    pub async fn in_temp_dir() -> Result<Self, PersistenceError> {
        let temp_dir = TempDir::new("yams-persistence").to_persistence()?;
        let path = temp_dir.path().join("yams.db");
        Ok(Self {
            variant: InstanceType::TempDir {
                db: libsql::Builder::new_local(path)
                    .build()
                    .await
                    .to_persistence()?,
                temp_dir: Arc::new(temp_dir),
            },
        })
    }

    pub async fn in_memory() -> Result<Self, PersistenceError> {
        let memory_uuid = Uuid::new_v4();
        let db =
            libsql::Builder::new_local(format!("file:{}.db?mode=memory&cache=shared", memory_uuid))
                .build()
                .await
                .to_persistence()?;
        Ok(Self {
            variant: InstanceType::InMemory {
                connection: Arc::new(
                    Self::initialize_connection(db.connect().to_persistence()?).await?,
                ),
                connection_lock: Arc::new(Mutex::new(())),
            },
        })
    }

    pub async fn migrate_to_latest(&mut self) -> Result<(), PersistenceError> {
        let mut connection = self.create_connection().await?;
        MIGRATIONS
            .apply(&mut connection, None)
            .await
            .to_persistence()
    }

    pub(crate) async fn create_connection(&self) -> Result<SQLiteConnection, PersistenceError> {
        match &self.variant {
            InstanceType::Local(db) => {
                let connection = db.connect().to_persistence()?;
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
                let connection = db.connect().to_persistence()?;
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
    ) -> Result<libsql::Connection, PersistenceError> {
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
            connection.query(statement, ()).await.to_persistence()?;
        }
        Ok(connection)
    }
}
