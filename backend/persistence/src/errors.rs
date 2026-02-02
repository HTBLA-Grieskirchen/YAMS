use anyhow::anyhow;
use molting::MigrationError;
use std::io;
use yams_core::service::errors::PersistenceError;

pub fn libsql_error_to_persistence_error(e: libsql::Error) -> PersistenceError {
    match e {
        libsql::Error::ConnectionFailed(msg) => {
            PersistenceError::ConnectionError(io::Error::new(io::ErrorKind::ConnectionRefused, msg))
        }
        libsql::Error::SqliteFailure(_code, msg) => {
            PersistenceError::TransactionFailed(anyhow::Error::msg(msg))
        }
        libsql::Error::NullValue => {
            PersistenceError::DeserializationError(anyhow::Error::msg("null value"))
        }
        libsql::Error::Misuse(msg) => PersistenceError::Unknown(anyhow::Error::msg(msg)),
        libsql::Error::ExecuteReturnedRows => {
            PersistenceError::Unknown(anyhow::Error::msg("execute returned rows"))
        }
        libsql::Error::QueryReturnedNoRows => PersistenceError::NotFound,
        libsql::Error::InvalidColumnName(name) => PersistenceError::DeserializationError(
            anyhow::Error::msg(format!("invalid column name: {name}")),
        ),
        libsql::Error::ToSqlConversionFailure(e) => {
            PersistenceError::SerializationError(anyhow::Error::msg(e.to_string()))
        }
        libsql::Error::SyncNotSupported(_)
        | libsql::Error::LoadExtensionNotSupported
        | libsql::Error::AuthorizerNotSupported
        | libsql::Error::UpdateHookNotSupported => PersistenceError::PermissionDenied,
        libsql::Error::ColumnNotFound(_) => PersistenceError::NotFound,
        libsql::Error::Hrana(e) => {
            PersistenceError::TransactionFailed(anyhow::Error::msg(e.to_string()))
        }
        libsql::Error::WriteDelegation(e) => {
            PersistenceError::TransactionFailed(anyhow::Error::msg(e.to_string()))
        }
        libsql::Error::Bincode(e) => {
            PersistenceError::DeserializationError(anyhow::Error::msg(e.to_string()))
        }
        libsql::Error::InvalidColumnIndex | libsql::Error::InvalidColumnType => {
            PersistenceError::DeserializationError(anyhow::Error::msg(
                "invalid column index or type",
            ))
        }
        libsql::Error::Sqlite3SyntaxError(_, _, msg) => {
            PersistenceError::TransactionFailed(anyhow::Error::msg(msg))
        }
        libsql::Error::Sqlite3UnsupportedStatement => {
            PersistenceError::TransactionFailed(anyhow::Error::msg("unsupported statement"))
        }
        libsql::Error::Sqlite3ParserError(e) => {
            PersistenceError::TransactionFailed(anyhow::Error::msg(e.to_string()))
        }
        libsql::Error::RemoteSqliteFailure(_, _, msg) => {
            PersistenceError::TransactionFailed(anyhow::Error::msg(msg))
        }
        libsql::Error::Replication(e) => {
            PersistenceError::TransactionFailed(anyhow::Error::msg(e.to_string()))
        }
        libsql::Error::InvalidUTF8Path => {
            PersistenceError::Unknown(anyhow::Error::msg("path has invalid UTF-8"))
        }
        libsql::Error::FreezeNotSupported(msg) | libsql::Error::InvalidParserState(msg) => {
            PersistenceError::Unknown(anyhow::Error::msg(msg))
        }
        libsql::Error::InvalidTlsConfiguration(io_err) => PersistenceError::ConnectionError(io_err),
        libsql::Error::TransactionalBatchError(msg) => {
            PersistenceError::TransactionFailed(anyhow::Error::msg(msg))
        }
        libsql::Error::InvalidBlobSize(_) => {
            PersistenceError::DeserializationError(anyhow::Error::msg("invalid blob size"))
        }
        libsql::Error::Sync(e) => {
            PersistenceError::TransactionFailed(anyhow::Error::msg(e.to_string()))
        }
        libsql::Error::WalConflict => PersistenceError::ConcurrentModification,
        libsql::Error::ReservedBytesNotSupported => {
            PersistenceError::Unknown(anyhow::Error::msg("reserved bytes not supported"))
        }
        _ => PersistenceError::Unknown(anyhow::Error::new(e)),
    }
}

pub trait ToPersistenceResultExt<T> {
    fn to_persistence(self) -> Result<T, PersistenceError>;
}

impl<T> ToPersistenceResultExt<T> for Result<T, libsql::Error> {
    fn to_persistence(self) -> Result<T, PersistenceError> {
        self.map_err(libsql_error_to_persistence_error)
    }
}

impl<T> ToPersistenceResultExt<T> for Result<T, MigrationError<libsql::Error>> {
    fn to_persistence(self) -> Result<T, PersistenceError> {
        self.map_err(|e| match e {
            MigrationError::RunnerError(e) => libsql_error_to_persistence_error(e),
            MigrationError::MigrationFailed { id, source } => {
                PersistenceError::Unknown(anyhow!(MigrationError::MigrationFailed { id, source }))
            }
            MigrationError::VersionMismatch { expected, actual } => {
                PersistenceError::DataCorruption(anyhow!(MigrationError::VersionMismatch::<
                    libsql::Error,
                > {
                    expected,
                    actual
                }))
            }
            MigrationError::DownMigrationNotSupported => PersistenceError::Unknown(anyhow!(
                MigrationError::DownMigrationNotSupported::<libsql::Error>
            )),
        })
    }
}
