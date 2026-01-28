pub trait ServiceError {
    fn should_retry(&self) -> bool;
}

impl ServiceError for ! {
    fn should_retry(&self) -> bool {
        false
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PersistenceError {
    #[error("entity not found")]
    NotFound,
    #[error("version mismatch - entity was modified by another process")]
    VersionMismatch { expected: i64, actual: i64 },
    #[error("concurrent modification detected")]
    ConcurrentModification,
    #[error("connection error")]
    ConnectionError(#[source] std::io::Error),
    #[error("transaction failed")]
    TransactionFailed(#[source] anyhow::Error),
    #[error("constraint violation")]
    ConstraintViolation(#[source] anyhow::Error),
    #[error("serialization error")]
    SerializationError(#[source] anyhow::Error),
    #[error("deserialization error")]
    DeserializationError(#[source] anyhow::Error),
    #[error("timeout occurred")]
    Timeout,
    #[error("insufficient permissions")]
    PermissionDenied,
    #[error("storage quota exceeded")]
    QuotaExceeded,
    #[error("data corruption detected")]
    DataCorruption(#[source] anyhow::Error),
    #[error("lock acquisition failed")]
    LockFailed,
    #[error("migration required")]
    MigrationRequired,
    #[error("unknown persistence error")]
    Unknown(#[source] anyhow::Error),
}

impl ServiceError for PersistenceError {
    fn should_retry(&self) -> bool {
        match self {
            PersistenceError::NotFound => false,
            PersistenceError::VersionMismatch { .. } => true,
            PersistenceError::ConcurrentModification => true,
            PersistenceError::ConnectionError(_) => true,
            PersistenceError::TransactionFailed(_) => true,
            PersistenceError::ConstraintViolation(_) => false,
            PersistenceError::SerializationError(_) => false,
            PersistenceError::DeserializationError(_) => false,
            PersistenceError::Timeout => true,
            PersistenceError::PermissionDenied => false,
            PersistenceError::QuotaExceeded => false,
            PersistenceError::DataCorruption(_) => false,
            PersistenceError::LockFailed => true,
            PersistenceError::MigrationRequired => false,
            PersistenceError::Unknown(_) => false,
        }
    }
}
