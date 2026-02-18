use error_stack::{IntoReport, Report, ResultExt};

pub trait StableError: std::error::Error + Send + Sync + 'static {}

impl<E: std::error::Error + Send + Sync + 'static> StableError for E {}

#[derive(thiserror::Error, Debug)]
pub enum PersistenceError {
    #[error("entity does not exist")]
    NotFound,
    #[error("version mismatch - entity was modified by another process")]
    VersionMismatch { expected: u64, actual: Option<u64> },
    #[error("concurrent modification detected")]
    ConcurrentModification,
    #[error("connection error")]
    ConnectionError,
    #[error("transaction failed")]
    TransactionFailed,
    #[error("constraint violation")]
    ConstraintViolation,
    #[error("serialization error")]
    SerializationError,
    #[error("deserialization error")]
    DeserializationError,
    #[error("timeout occurred")]
    Timeout,
    #[error("insufficient permissions")]
    PermissionDenied,
    #[error("storage quota exceeded")]
    QuotaExceeded,
    #[error("data corruption detected")]
    DataCorruption,
    #[error("lock acquisition failed")]
    LockFailed,
    #[error("migration required")]
    MigrationRequired,
    #[error("unknown persistence error")]
    Unknown,
}

pub struct MarkShouldRetry;

pub trait ErrorReportExt<T, E: StableError> {
    fn contextualize<C: StableError>(self, context: C) -> Result<T, Report<C>>;

    fn contextualize_with<C: StableError>(
        self,
        context_fn: impl Fn(&E) -> C,
    ) -> Result<T, Report<C>>;

    fn reportize(self) -> Result<T, Report<E>>;
}

impl<T, E: StableError> ErrorReportExt<T, E> for Result<T, E> {
    fn contextualize<C: StableError>(self, context: C) -> Result<T, Report<C>> {
        self.map_err(IntoReport::into_report)
            .change_context(context)
    }

    fn contextualize_with<C: StableError>(
        self,
        context_fn: impl Fn(&E) -> C,
    ) -> Result<T, Report<C>> {
        self.map_err(IntoReport::into_report).map_err(|e| {
            let new_context = context_fn(e.current_context());
            e.change_context(new_context)
        })
    }

    fn reportize(self) -> Result<T, Report<E>> {
        self.map_err(IntoReport::into_report)
    }
}

pub type CoreResult<T, E> = Result<T, Report<E>>;
