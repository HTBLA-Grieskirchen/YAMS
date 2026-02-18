use molting::MigrationError;
use yams_core::service::errors::PersistenceError;

pub fn libsql_error_to_persistence_error(e: &libsql::Error) -> PersistenceError {
    match e {
        libsql::Error::ConnectionFailed(_msg) => PersistenceError::ConnectionError,
        libsql::Error::SqliteFailure(_code, _msg) => PersistenceError::TransactionFailed,
        libsql::Error::NullValue => PersistenceError::DeserializationError,
        libsql::Error::Misuse(_msg) => PersistenceError::Unknown,
        libsql::Error::ExecuteReturnedRows => PersistenceError::Unknown,
        libsql::Error::QueryReturnedNoRows => PersistenceError::NotFound,
        libsql::Error::InvalidColumnName(_name) => PersistenceError::DeserializationError,
        libsql::Error::ToSqlConversionFailure(_e) => PersistenceError::SerializationError,
        libsql::Error::SyncNotSupported(_)
        | libsql::Error::LoadExtensionNotSupported
        | libsql::Error::AuthorizerNotSupported
        | libsql::Error::UpdateHookNotSupported => PersistenceError::PermissionDenied,
        libsql::Error::ColumnNotFound(_) => PersistenceError::NotFound,
        libsql::Error::Hrana(_e) => PersistenceError::TransactionFailed,
        libsql::Error::WriteDelegation(_e) => PersistenceError::TransactionFailed,
        libsql::Error::Bincode(_e) => PersistenceError::DeserializationError,
        libsql::Error::InvalidColumnIndex | libsql::Error::InvalidColumnType => {
            PersistenceError::DeserializationError
        }
        libsql::Error::Sqlite3SyntaxError(_, _, _msg) => PersistenceError::TransactionFailed,
        libsql::Error::Sqlite3UnsupportedStatement => PersistenceError::TransactionFailed,
        libsql::Error::Sqlite3ParserError(_e) => PersistenceError::TransactionFailed,
        libsql::Error::RemoteSqliteFailure(_, _, _msg) => PersistenceError::TransactionFailed,
        libsql::Error::Replication(_e) => PersistenceError::TransactionFailed,
        libsql::Error::InvalidUTF8Path => PersistenceError::Unknown,
        libsql::Error::FreezeNotSupported(_msg) | libsql::Error::InvalidParserState(_msg) => {
            PersistenceError::Unknown
        }
        libsql::Error::InvalidTlsConfiguration(_io_err) => PersistenceError::ConnectionError,
        libsql::Error::TransactionalBatchError(_msg) => PersistenceError::TransactionFailed,
        libsql::Error::InvalidBlobSize(_) => PersistenceError::DeserializationError,
        libsql::Error::Sync(_e) => PersistenceError::TransactionFailed,
        libsql::Error::WalConflict => PersistenceError::ConcurrentModification,
        libsql::Error::ReservedBytesNotSupported => PersistenceError::Unknown,
        _ => PersistenceError::Unknown,
    }
}

pub fn migration_error_to_persistence_error(e: &MigrationError<libsql::Error>) -> PersistenceError {
    match e {
        MigrationError::RunnerError(e) => libsql_error_to_persistence_error(e),
        MigrationError::MigrationFailed { id: _, source: _ } => PersistenceError::Unknown,
        MigrationError::VersionMismatch {
            expected: _,
            actual: _,
        } => PersistenceError::DataCorruption,
        MigrationError::DownMigrationNotSupported => PersistenceError::Unknown,
    }
}
