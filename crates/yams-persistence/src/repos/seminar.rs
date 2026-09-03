use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use chrono::TimeDelta;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;
use yams_core::{
    ErrorReportExt,
    domain::{Seminar, SeminarId, seminar::NeuesSeminar},
    ports::{RepositoryError, RepositoryResult, SeminarRepository},
    uow::Versioned,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{parse_preis, parse_ratio, parse_uuid, preis_to_str, ratio_to_str};

pub struct SQLiteSeminarRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn seminar_from_row(row: &Row) -> RepositoryResult<Versioned<Seminar>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let titel: String = row.get(1).contextualize(RepositoryError::Data)?;
    let beschreibung: String = row.get(2).contextualize(RepositoryError::Data)?;
    let basis_str: String = row.get(3).contextualize(RepositoryError::Data)?;
    let mwst_str: String = row.get(4).contextualize(RepositoryError::Data)?;
    let standarddauer_ms: Option<i64> = row.get(5).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(6).contextualize(RepositoryError::Data)?;

    let uuid = parse_uuid(&id_raw)?;
    let basis = parse_preis(&basis_str)?;
    let mwst = parse_ratio(&mwst_str)?;
    let standarddauer = standarddauer_ms.map(TimeDelta::milliseconds);

    let seminar = Seminar::from_parts(
        SeminarId(uuid),
        titel,
        beschreibung,
        basis,
        mwst,
        standarddauer,
    )
    .change_context(RepositoryError::Data)?;
    Ok(Versioned::new(version, seminar))
}

#[async_trait]
impl SeminarRepository for SQLiteSeminarRepository {
    async fn find_by_id(&self, id: SeminarId) -> RepositoryResult<Versioned<Seminar>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, titel, beschreibung, \"teilnahmegebühr_basis\", mwst, standarddauer_ms, _version FROM seminare WHERE id = ?1",
                [id_str],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::NotFound)?;

        seminar_from_row(&row)
    }

    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Seminar>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                "SELECT id, titel, beschreibung, \"teilnahmegebühr_basis\", mwst, standarddauer_ms, _version FROM seminare ORDER BY titel",
                (),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut seminare = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            seminare.push(seminar_from_row(&row)?);
        }
        Ok(seminare)
    }

    async fn create(&self, new: NeuesSeminar) -> RepositoryResult<Versioned<Seminar>> {
        let id = SeminarId(Uuid::new_v4());
        let seminar = Versioned::init(Seminar::neu(id, new).change_context(RepositoryError::Data)?);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO seminare (id, titel, beschreibung, \"teilnahmegebühr_basis\", mwst, standarddauer_ms, _version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            libsql::params![
                seminar.id().0.to_string(),
                seminar.titel(),
                seminar.beschreibung(),
                preis_to_str(seminar.teilnahmegebühr_basis()),
                ratio_to_str(seminar.mwst()),
                seminar.standarddauer().map(|d| d.num_milliseconds()),
                seminar.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(seminar)
    }

    async fn update(&self, seminar: &mut Versioned<Seminar>) -> RepositoryResult<()> {
        let version = seminar.v();
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "UPDATE seminare SET titel = ?1, beschreibung = ?2, \"teilnahmegebühr_basis\" = ?3, mwst = ?4, standarddauer_ms = ?5, _version = _version + 1 WHERE id = ?6 AND _version = ?7",
                libsql::params![
                    seminar.titel(),
                    seminar.beschreibung(),
                    preis_to_str(seminar.teilnahmegebühr_basis()),
                    ratio_to_str(seminar.mwst()),
                    seminar.standarddauer().map(|d| d.num_milliseconds()),
                    seminar.id().0.to_string(),
                    version,
                ],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if result != 1 {
            Err(RepositoryError::VersionMismatch {
                expected: version,
                actual: None,
            })?;
        }

        seminar.increment();
        Ok(())
    }
}
