use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;
use yams_core::{
    ErrorReportExt,
    domain::{Produkt, ProduktId, produkt::NeuesProdukt},
    ports::{ProduktRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{parse_preis, parse_ratio, parse_uuid, preis_to_str, ratio_to_str};

pub struct SQLiteProduktRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn produkt_from_row(row: &Row) -> RepositoryResult<Versioned<Produkt>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let name: String = row.get(1).contextualize(RepositoryError::Data)?;
    let beschreibung: String = row.get(2).contextualize(RepositoryError::Data)?;
    let einzelpreis_str: String = row.get(3).contextualize(RepositoryError::Data)?;
    let mwst_str: String = row.get(4).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(5).contextualize(RepositoryError::Data)?;

    let uuid = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let einzelpreis = parse_preis(&einzelpreis_str)?;
    let mwst = parse_ratio(&mwst_str)?;

    let produkt = Produkt::from_parts(ProduktId(uuid), name, beschreibung, einzelpreis, mwst)
        .change_context(RepositoryError::Data)?;
    Ok(Versioned::new(version, produkt))
}

#[async_trait]
impl ProduktRepository for SQLiteProduktRepository {
    async fn find_by_id(&self, id: ProduktId) -> RepositoryResult<Versioned<Produkt>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, name, beschreibung, einzelpreis, mwst, _version FROM produkte WHERE id = ?1",
                [id_str],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::NotFound)?;

        produkt_from_row(&row)
    }

    async fn create(&self, new: NeuesProdukt) -> RepositoryResult<Versioned<Produkt>> {
        let id = ProduktId(Uuid::new_v4());
        let produkt = Versioned::init(Produkt::neu(id, new));

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO produkte (id, name, beschreibung, einzelpreis, mwst, _version) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            libsql::params![
                produkt.id().0.to_string(),
                produkt.name(),
                produkt.beschreibung(),
                preis_to_str(produkt.einzelpreis()),
                ratio_to_str(produkt.mwst()),
                produkt.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(produkt)
    }
}
