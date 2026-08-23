use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::{Row, Transaction};
use uuid::Uuid;
use yams_core::{
    domain::{Behandlung, BehandlungId, behandlung::NeueBehandlung},
    ports::{BehandlungRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
    ErrorReportExt,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{parse_preis, parse_uuid, preis_to_str};

pub struct SQLiteBehandlungRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn behandlung_from_row(row: &Row) -> RepositoryResult<Versioned<Behandlung>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let name: String = row.get(1).contextualize(RepositoryError::Data)?;
    let beschreibung: String = row.get(2).contextualize(RepositoryError::Data)?;
    let standardpreis_str: String = row.get(3).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(4).contextualize(RepositoryError::Data)?;

    let uuid = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let standardpreis = parse_preis(&standardpreis_str)?;

    let behandlung = Behandlung {
        id: BehandlungId(uuid),
        name,
        beschreibung,
        standardpreis,
    };
    Ok(Versioned::new(version, behandlung))
}

#[async_trait]
impl BehandlungRepository for SQLiteBehandlungRepository {
    async fn find_by_id(&self, id: BehandlungId) -> RepositoryResult<Versioned<Behandlung>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, name, beschreibung, standardpreis, _version FROM behandlungen WHERE id = ?1",
                [id_str],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::NotFound)?;

        behandlung_from_row(&row)
    }

    async fn create(&self, new: NeueBehandlung) -> RepositoryResult<Versioned<Behandlung>> {
        let id = BehandlungId(Uuid::new_v4());
        let behandlung = Behandlung {
            id,
            name: new.name,
            beschreibung: new.beschreibung,
            standardpreis: new.standardpreis,
        };
        let behandlung = Versioned::init(behandlung);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO behandlungen (id, name, beschreibung, standardpreis, _version) VALUES (?1, ?2, ?3, ?4, ?5)",
            libsql::params![
                behandlung.id.0.to_string(),
                behandlung.name.clone(),
                behandlung.beschreibung.clone(),
                preis_to_str(&behandlung.standardpreis),
                behandlung.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(behandlung)
    }
}
