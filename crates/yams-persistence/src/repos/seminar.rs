use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::Transaction;
use yams_core::{
    domain::{Seminar, SeminarId, seminar::NeuesSeminar},
    ports::{RepositoryError, RepositoryResult, SeminarRepository},
    uow::Versioned,
};

#[allow(dead_code)]
pub struct SQLiteSeminarRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl SeminarRepository for SQLiteSeminarRepository {
    async fn find_by_id(&self, _id: SeminarId) -> RepositoryResult<Versioned<Seminar>> {
        Err(RepositoryError::OperationFailed)?
    }

    async fn create(&self, _seminar: NeuesSeminar) -> RepositoryResult<Versioned<Seminar>> {
        Err(RepositoryError::OperationFailed)?
    }

    async fn update(&self, _seminar: &mut Versioned<Seminar>) -> RepositoryResult<()> {
        Err(RepositoryError::OperationFailed)?
    }
}
