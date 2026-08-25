use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;
use yams_core::{
    ErrorReportExt,
    domain::{Haustier, HaustierId, KlientId, haustier::NeuesHaustier},
    ports::{HaustierRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{format_naive_date, parse_klient_id, parse_naive_date, parse_uuid};

pub struct SQLiteHaustierRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn haustier_from_row(row: &Row) -> RepositoryResult<Versioned<Haustier>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let name: String = row.get(1).contextualize(RepositoryError::Data)?;
    let geburtstag_str: String = row.get(2).contextualize(RepositoryError::Data)?;
    let tierart: String = row.get(3).contextualize(RepositoryError::Data)?;
    let beschreibung: String = row.get(4).contextualize(RepositoryError::Data)?;
    let klient_id_str: String = row.get(5).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(6).contextualize(RepositoryError::Data)?;

    let geburtstag = parse_naive_date(&geburtstag_str).contextualize(RepositoryError::Data)?;
    let uuid = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let klient_id = parse_klient_id(&klient_id_str)?;

    let haustier = Haustier::from_parts(
        HaustierId(uuid),
        klient_id,
        name,
        geburtstag,
        tierart,
        beschreibung,
    )
    .change_context(RepositoryError::Data)?;
    Ok(Versioned::new(version, haustier))
}

async fn query_all_haustiere(
    tx: &mut libsql::Transaction,
    sql: &str,
    params: impl libsql::params::IntoParams,
) -> RepositoryResult<Vec<Versioned<Haustier>>> {
    let mut rows = tx
        .query(sql, params)
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

    let mut haustiere = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .contextualize_with(libsql_error_to_persistence_error)?
    {
        haustiere.push(haustier_from_row(&row)?);
    }
    Ok(haustiere)
}

#[async_trait]
impl HaustierRepository for SQLiteHaustierRepository {
    async fn find_by_id(&self, id: HaustierId) -> RepositoryResult<Versioned<Haustier>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, name, geburtstag, tierart, beschreibung, klient_id, _version FROM haustiere WHERE id = ?1",
                [id_str],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::NotFound)?;

        haustier_from_row(&row)
    }

    async fn find_by_klient_id(
        &self,
        klient_id: KlientId,
    ) -> RepositoryResult<Vec<Versioned<Haustier>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        query_all_haustiere(
            tx,
            "SELECT id, name, geburtstag, tierart, beschreibung, klient_id, _version FROM haustiere WHERE klient_id = ?1",
            [klient_id.0.to_string()],
        )
        .await
    }

    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Haustier>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        query_all_haustiere(
            tx,
            "SELECT id, name, geburtstag, tierart, beschreibung, klient_id, _version FROM haustiere",
            (),
        )
        .await
    }

    async fn create(&self, new: NeuesHaustier) -> RepositoryResult<Versioned<Haustier>> {
        let id = HaustierId(Uuid::new_v4());
        let haustier =
            Versioned::init(Haustier::neu(id, new).change_context(RepositoryError::Data)?);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO haustiere (id, name, geburtstag, tierart, beschreibung, klient_id, _version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            libsql::params![
                haustier.id().0.to_string(),
                haustier.name(),
                format_naive_date(haustier.geburtstag()),
                haustier.tierart(),
                haustier.beschreibung(),
                haustier.klient_id().0.to_string(),
                haustier.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(haustier)
    }

    async fn update(&self, haustier: &mut Versioned<Haustier>) -> RepositoryResult<()> {
        let id_str = haustier.id().0.to_string();
        let version = haustier.v();

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "UPDATE haustiere SET name = ?1, geburtstag = ?2, tierart = ?3, beschreibung = ?4, klient_id = ?5, _version = _version + 1 WHERE id = ?6 AND _version = ?7",
                libsql::params![
                    haustier.name(),
                    format_naive_date(haustier.geburtstag()),
                    haustier.tierart(),
                    haustier.beschreibung(),
                    haustier.klient_id().0.to_string(),
                    id_str,
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

        haustier.increment();
        Ok(())
    }

    async fn delete(&self, haustier: Versioned<Haustier>) -> RepositoryResult<()> {
        let id_str = haustier.id().0.to_string();
        let version = haustier.v();

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "DELETE FROM haustiere WHERE id = ?1 AND _version = ?2",
                libsql::params![id_str, version],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if result != 1 {
            Err(RepositoryError::VersionMismatch {
                expected: version,
                actual: None,
            })?;
        }
        Ok(())
    }
}
