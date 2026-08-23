use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::{Row, Transaction};
use uuid::Uuid;
use yams_core::{
    domain::{
        KlientId, LeistungId, Rechnung, RechnungId, rechnung::Rechnungsposition,
    },
    ports::{RechnungRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
    ErrorReportExt,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{
    format_naive_date, parse_klient_id, parse_naive_date, parse_preis, parse_uuid, preis_to_str,
    rechnung_status_from_str, rechnung_status_to_str,
};

pub struct SQLiteRechnungRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

async fn load_positionen(
    tx: &mut libsql::Transaction,
    rechnung_id: &str,
) -> RepositoryResult<Vec<Rechnungsposition>> {
    let mut rows = tx
        .query(
            "SELECT leistung_id, beschreibung, betrag FROM rechnungspositionen WHERE rechnung_id = ?1",
            [rechnung_id],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

    let mut positionen = Vec::new();
    while let Some(row) = rows
        .next()
        .await
        .contextualize_with(libsql_error_to_persistence_error)?
    {
        let leistung_id_str: String = row.get(0).contextualize(RepositoryError::Data)?;
        let beschreibung: String = row.get(1).contextualize(RepositoryError::Data)?;
        let betrag_str: String = row.get(2).contextualize(RepositoryError::Data)?;

        let leistung_uuid = parse_uuid(&leistung_id_str).contextualize(RepositoryError::Data)?;
        positionen.push(Rechnungsposition {
            leistung_id: LeistungId(leistung_uuid),
            beschreibung,
            betrag: parse_preis(&betrag_str)?,
        });
    }
    Ok(positionen)
}

fn rechnung_from_row(row: &Row, positionen: Vec<Rechnungsposition>) -> RepositoryResult<Versioned<Rechnung>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let rechnungsnummer: i64 = row.get(1).contextualize(RepositoryError::Data)?;
    let klient_id_str: String = row.get(2).contextualize(RepositoryError::Data)?;
    let rechnungsdatum_str: String = row.get(3).contextualize(RepositoryError::Data)?;
    let gesamtbetrag_str: String = row.get(4).contextualize(RepositoryError::Data)?;
    let status_str: String = row.get(5).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(6).contextualize(RepositoryError::Data)?;

    let uuid = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let klient_id = parse_klient_id(&klient_id_str)?;
    let rechnungsdatum =
        parse_naive_date(&rechnungsdatum_str).contextualize(RepositoryError::Data)?;
    let gesamtbetrag = parse_preis(&gesamtbetrag_str)?;
    let status = rechnung_status_from_str(&status_str)?;

    let rechnung = Rechnung {
        id: RechnungId(uuid),
        rechnungsnummer,
        klient_id,
        rechnungsdatum,
        positionen,
        gesamtbetrag,
        status,
    };
    Ok(Versioned::new(version, rechnung))
}

#[async_trait]
impl RechnungRepository for SQLiteRechnungRepository {
    async fn create(&self, rechnung: Rechnung) -> RepositoryResult<Versioned<Rechnung>> {
        let rechnung = Versioned::init(rechnung);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO rechnungen (id, rechnungsnummer, klient_id, rechnungsdatum, gesamtbetrag, status, _version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            libsql::params![
                rechnung.id.0.to_string(),
                rechnung.rechnungsnummer,
                rechnung.klient_id.0.to_string(),
                format_naive_date(rechnung.rechnungsdatum),
                preis_to_str(&rechnung.gesamtbetrag),
                rechnung_status_to_str(&rechnung.status),
                rechnung.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        for position in &rechnung.positionen {
            tx.execute(
                "INSERT INTO rechnungspositionen (id, rechnung_id, leistung_id, beschreibung, betrag) VALUES (?1, ?2, ?3, ?4, ?5)",
                libsql::params![
                    Uuid::new_v4().to_string(),
                    rechnung.id.0.to_string(),
                    position.leistung_id.0.to_string(),
                    position.beschreibung.clone(),
                    preis_to_str(&position.betrag),
                ],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        }

        Ok(rechnung)
    }

    async fn naechste_rechnungsnummer(&self) -> RepositoryResult<i64> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let mut rows = tx
            .query(
                "SELECT COALESCE(MAX(rechnungsnummer), 0) + 1 FROM rechnungen",
                (),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::Data)?;

        let nummer: i64 = row.get(0).contextualize(RepositoryError::Data)?;
        Ok(nummer)
    }

    async fn find_by_klient_id(
        &self,
        klient_id: KlientId,
    ) -> RepositoryResult<Vec<Versioned<Rechnung>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let klient_id_str = klient_id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, rechnungsnummer, klient_id, rechnungsdatum, gesamtbetrag, status, _version FROM rechnungen WHERE klient_id = ?1",
                [klient_id_str],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut rechnungen = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
            let positionen = load_positionen(tx, &id_raw).await?;
            rechnungen.push(rechnung_from_row(&row, positionen)?);
        }
        Ok(rechnungen)
    }
}
