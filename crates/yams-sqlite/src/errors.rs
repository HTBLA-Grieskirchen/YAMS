use molting::MigrationError;
use yams_core::ports::RepositoryError;

pub fn libsql_error_to_persistence_error(e: &libsql::Error) -> RepositoryError {
    match e {
        libsql::Error::ConnectionFailed(_msg) => RepositoryError::Connection,
        libsql::Error::SqliteFailure(_code, _msg) => RepositoryError::OperationFailed,
        libsql::Error::NullValue => RepositoryError::Data,
        libsql::Error::Misuse(_msg) => RepositoryError::Unknown,
        libsql::Error::ExecuteReturnedRows => RepositoryError::Unknown,
        libsql::Error::QueryReturnedNoRows => RepositoryError::NotFound,
        libsql::Error::InvalidColumnName(_name) => RepositoryError::Data,
        libsql::Error::ToSqlConversionFailure(_e) => RepositoryError::Data,
        libsql::Error::SyncNotSupported(_)
        | libsql::Error::LoadExtensionNotSupported
        | libsql::Error::AuthorizerNotSupported
        | libsql::Error::UpdateHookNotSupported => RepositoryError::OperationFailed,
        libsql::Error::ColumnNotFound(_) => RepositoryError::NotFound,
        libsql::Error::Hrana(_e) => RepositoryError::OperationFailed,
        libsql::Error::WriteDelegation(_e) => RepositoryError::OperationFailed,
        libsql::Error::Bincode(_e) => RepositoryError::Data,
        libsql::Error::InvalidColumnIndex | libsql::Error::InvalidColumnType => {
            RepositoryError::Data
        }
        libsql::Error::Sqlite3SyntaxError(_, _, _msg) => RepositoryError::OperationFailed,
        libsql::Error::Sqlite3UnsupportedStatement => RepositoryError::OperationFailed,
        libsql::Error::Sqlite3ParserError(_e) => RepositoryError::OperationFailed,
        libsql::Error::RemoteSqliteFailure(_, _, _msg) => RepositoryError::OperationFailed,
        libsql::Error::Replication(_e) => RepositoryError::OperationFailed,
        libsql::Error::InvalidUTF8Path => RepositoryError::Unknown,
        libsql::Error::FreezeNotSupported(_msg) | libsql::Error::InvalidParserState(_msg) => {
            RepositoryError::Unknown
        }
        libsql::Error::InvalidTlsConfiguration(_io_err) => RepositoryError::Connection,
        libsql::Error::TransactionalBatchError(_msg) => RepositoryError::OperationFailed,
        libsql::Error::InvalidBlobSize(_) => RepositoryError::Data,
        libsql::Error::Sync(_e) => RepositoryError::Connection,
        libsql::Error::WalConflict => RepositoryError::Conflict,
        libsql::Error::ReservedBytesNotSupported => RepositoryError::Unknown,
        _ => RepositoryError::Unknown,
    }
}

pub fn migration_error_to_persistence_error(e: &MigrationError<libsql::Error>) -> RepositoryError {
    match e {
        MigrationError::RunnerError(e) => libsql_error_to_persistence_error(e),
        MigrationError::MigrationFailed { id: _, source: _ } => RepositoryError::Unknown,
        MigrationError::VersionMismatch {
            expected: _,
            actual: _,
        } => RepositoryError::Conflict,
        MigrationError::DownMigrationNotSupported => RepositoryError::Unknown,
    }
}
