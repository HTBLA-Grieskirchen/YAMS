use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;
use yams_core::{
    domain::{Leistung, LeistungId, leistung::NeueLeistung, RechnungId},
    ports::{LeistungRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
    ErrorReportExt,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{
    format_naive_date, leistung_status_from_str, leistung_status_to_str, parse_klient_id,
    parse_naive_date, parse_preis, parse_rechnung_id, parse_uuid, preis_to_str, quelle_from_row,
    quelle_to_db,
};

pub struct SQLiteLeistungRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn leistung_from_row(row: &Row) -> RepositoryResult<Versioned<Leistung>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let klient_id_str: String = row.get(1).contextualize(RepositoryError::Data)?;
    let haustier_id_str: Option<String> = row.get(2).contextualize(RepositoryError::Data)?;
    let beschreibung: String = row.get(3).contextualize(RepositoryError::Data)?;
    let betrag_str: String = row.get(4).contextualize(RepositoryError::Data)?;
    let leistungsdatum_str: String = row.get(5).contextualize(RepositoryError::Data)?;
    let status_str: String = row.get(6).contextualize(RepositoryError::Data)?;
    let quelle_typ: String = row.get(7).contextualize(RepositoryError::Data)?;
    let quelle_id: Option<String> = row.get(8).contextualize(RepositoryError::Data)?;
    let rechnung_id_str: Option<String> = row.get(9).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(10).contextualize(RepositoryError::Data)?;

    let uuid = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;
    let klient_id = parse_klient_id(&klient_id_str)?;
    let haustier_id = haustier_id_str
        .map(|s| super::common::parse_haustier_id(&s))
        .transpose()?;
    let betrag = parse_preis(&betrag_str)?;
    let leistungsdatum =
        parse_naive_date(&leistungsdatum_str).contextualize(RepositoryError::Data)?;
    let status = leistung_status_from_str(&status_str)?;
    let quelle = quelle_from_row(&quelle_typ, quelle_id)?;
    let rechnung_id = rechnung_id_str
        .map(|s| parse_rechnung_id(&s))
        .transpose()?;

    let leistung = Leistung {
        id: LeistungId(uuid),
        klient_id,
        haustier_id,
        beschreibung,
        betrag,
        leistungsdatum,
        status,
        quelle,
        rechnung_id,
    };
    Ok(Versioned::new(version, leistung))
}

#[async_trait]
impl LeistungRepository for SQLiteLeistungRepository {
    async fn create(&self, new: NeueLeistung) -> RepositoryResult<Versioned<Leistung>> {
        let id = LeistungId(Uuid::new_v4());
        let leistung = Leistung {
            id,
            klient_id: new.klient_id,
            haustier_id: new.haustier_id,
            beschreibung: new.beschreibung,
            betrag: new.betrag,
            leistungsdatum: new.leistungsdatum,
            status: yams_core::domain::LeistungStatus::Offen,
            quelle: new.quelle,
            rechnung_id: None,
        };
        let leistung = Versioned::init(leistung);
        let (quelle_typ, quelle_id) = quelle_to_db(&leistung.quelle);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO leistungen (id, klient_id, haustier_id, beschreibung, betrag, leistungsdatum, status, quelle_typ, quelle_id, rechnung_id, _version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, ?10)",
            libsql::params![
                leistung.id.0.to_string(),
                leistung.klient_id.0.to_string(),
                leistung.haustier_id.as_ref().map(|h| h.0.to_string()),
                leistung.beschreibung.clone(),
                preis_to_str(&leistung.betrag),
                format_naive_date(leistung.leistungsdatum),
                leistung_status_to_str(&leistung.status),
                quelle_typ,
                quelle_id,
                leistung.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(leistung)
    }

    async fn find_offene_by_datum(
        &self,
        datum: NaiveDate,
    ) -> RepositoryResult<Vec<Versioned<Leistung>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let datum_str = format_naive_date(datum);
        let mut rows = tx
            .query(
                "SELECT id, klient_id, haustier_id, beschreibung, betrag, leistungsdatum, status, quelle_typ, quelle_id, rechnung_id, _version FROM leistungen WHERE status = 'offen' AND leistungsdatum = ?1",
                [datum_str],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut leistungen = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            leistungen.push(leistung_from_row(&row)?);
        }
        Ok(leistungen)
    }

    async fn mark_abgerechnet(
        &self,
        id: LeistungId,
        rechnung_id: RechnungId,
    ) -> RepositoryResult<Versioned<Leistung>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, klient_id, haustier_id, beschreibung, betrag, leistungsdatum, status, quelle_typ, quelle_id, rechnung_id, _version FROM leistungen WHERE id = ?1",
                [id_str.clone()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::NotFound)?;

        let mut leistung = leistung_from_row(&row)?;
        let version = leistung.v();

        let rechnung_id_str = rechnung_id.0.to_string();

        leistung
            .mark_abgerechnet(rechnung_id)
            .change_context(RepositoryError::Data)?;

        let result = tx
            .execute(
                "UPDATE leistungen SET status = ?1, rechnung_id = ?2, _version = _version + 1 WHERE id = ?3 AND _version = ?4 AND status = 'offen'",
                libsql::params![
                    leistung_status_to_str(&leistung.status),
                    rechnung_id_str,
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

        leistung.increment();
        Ok(leistung)
    }
}
