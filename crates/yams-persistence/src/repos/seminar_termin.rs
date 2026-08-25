use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use chrono::NaiveDate;
use libsql::Transaction;
use yams_core::{
    domain::{
        SeminarId, SeminarTermin, SeminarTerminGeplant, SeminarTerminId,
        seminar_termin::NeuerSeminarTermin,
    },
    ports::{RepositoryError, RepositoryResult, SeminarTerminRepository},
    uow::Versioned,
};

#[allow(dead_code)]
pub struct SQLiteSeminarTerminRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

#[async_trait]
impl SeminarTerminRepository for SQLiteSeminarTerminRepository {
    async fn find_by_id(
        &self,
        _id: SeminarTerminId,
    ) -> RepositoryResult<Versioned<SeminarTermin>> {
        Err(RepositoryError::OperationFailed)?
    }

    async fn find_by_seminar_id(
        &self,
        _seminar_id: SeminarId,
    ) -> RepositoryResult<Vec<Versioned<SeminarTermin>>> {
        Err(RepositoryError::OperationFailed)?
    }

    async fn find_nicht_vollständig_abgerechnet_bis(
        &self,
        _stichtag: NaiveDate,
    ) -> RepositoryResult<Vec<Versioned<SeminarTermin>>> {
        Err(RepositoryError::OperationFailed)?
    }

    async fn create(
        &self,
        _termin: NeuerSeminarTermin,
    ) -> RepositoryResult<Versioned<SeminarTerminGeplant>> {
        Err(RepositoryError::OperationFailed)?
    }

    async fn update(&self, _termin: &mut Versioned<SeminarTermin>) -> RepositoryResult<()> {
        Err(RepositoryError::OperationFailed)?
    }
}
