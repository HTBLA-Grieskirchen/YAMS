use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;
use yams_core::{
    ErrorReportExt,
    domain::{Leistung, LeistungId, LeistungOffen, leistung::NeueLeistung},
    ports::{LeistungRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{
    format_naive_date, parse_klient_id, parse_naive_date, parse_rechnung_id, parse_uuid,
    quelle_from_row, quelle_to_db,
};

pub struct SQLiteLeistungRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

const LEISTUNG_SELECT: &str = "SELECT id, klient_id, haustier_id, beschreibung, leistungsdatum, quelle_typ, quelle_id, quelle_menge, quelle_einzelpreis, quelle_preis, quelle_mwst, rechnung_id, _version FROM leistungen";

fn leistung_from_row(row: &Row) -> RepositoryResult<Versioned<Leistung>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let klient_id_str: String = row.get(1).contextualize(RepositoryError::Data)?;
    let haustier_id_str: Option<String> = row.get(2).contextualize(RepositoryError::Data)?;
    let beschreibung: String = row.get(3).contextualize(RepositoryError::Data)?;
    let leistungsdatum_str: String = row.get(4).contextualize(RepositoryError::Data)?;
    let quelle_typ: String = row.get(5).contextualize(RepositoryError::Data)?;
    let quelle_id: Option<String> = row.get(6).contextualize(RepositoryError::Data)?;
    let quelle_menge: Option<String> = row.get(7).contextualize(RepositoryError::Data)?;
    let quelle_einzelpreis: Option<String> = row.get(8).contextualize(RepositoryError::Data)?;
    let quelle_preis: Option<String> = row.get(9).contextualize(RepositoryError::Data)?;
    let quelle_mwst: Option<String> = row.get(10).contextualize(RepositoryError::Data)?;
    let rechnung_id_str: Option<String> = row.get(11).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(12).contextualize(RepositoryError::Data)?;

    let uuid = parse_uuid(&id_raw)?;
    let klient_id = parse_klient_id(&klient_id_str)?;
    let haustier_id = haustier_id_str
        .map(|s| super::common::parse_haustier_id(&s))
        .transpose()?;
    let leistungsdatum =
        parse_naive_date(&leistungsdatum_str).contextualize(RepositoryError::Data)?;
    let quelle = quelle_from_row(
        &quelle_typ,
        quelle_id,
        quelle_menge,
        quelle_einzelpreis,
        quelle_preis,
        quelle_mwst,
    )?;
    let rechnung_id = rechnung_id_str.map(|s| parse_rechnung_id(&s)).transpose()?;

    let leistung = Leistung::from_parts(
        LeistungId(uuid),
        klient_id,
        haustier_id,
        beschreibung,
        leistungsdatum,
        quelle,
        rechnung_id,
    )
    .change_context(RepositoryError::Data)?;
    Ok(Versioned::new(version, leistung))
}

fn leistung_offen_from_row(row: &Row) -> RepositoryResult<Versioned<LeistungOffen>> {
    let versioned = leistung_from_row(row)?;
    let version = versioned.v();
    match versioned.into_data() {
        Leistung::Offen(offen) => Ok(Versioned::new(version, offen)),
        Leistung::Abgerechnet(_) => Err(RepositoryError::Data)?,
    }
}

#[async_trait]
impl LeistungRepository for SQLiteLeistungRepository {
    async fn create(&self, new: NeueLeistung) -> RepositoryResult<Versioned<LeistungOffen>> {
        let id = LeistungId(Uuid::new_v4());
        let offen = LeistungOffen::neu(id, new).change_context(RepositoryError::Data)?;
        let versioned = Versioned::init(offen.clone());
        let db = quelle_to_db(offen.quelle());

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO leistungen (id, klient_id, haustier_id, beschreibung, leistungsdatum, status, quelle_typ, quelle_id, quelle_menge, quelle_einzelpreis, quelle_preis, quelle_mwst, rechnung_id, _version) VALUES (?1, ?2, ?3, ?4, ?5, 'offen', ?6, ?7, ?8, ?9, ?10, ?11, NULL, ?12)",
            libsql::params![
                offen.id().0.to_string(),
                offen.klient_id().0.to_string(),
                offen.haustier_id().as_ref().map(|h| h.0.to_string()),
                offen.beschreibung(),
                format_naive_date(offen.leistungsdatum()),
                db.typ,
                db.id,
                db.menge,
                db.einzelpreis,
                db.preis,
                db.mwst,
                versioned.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(versioned)
    }

    async fn find_by_id(&self, id: LeistungId) -> RepositoryResult<Versioned<Leistung>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(&format!("{LEISTUNG_SELECT} WHERE id = ?1"), [id_str])
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::NotFound)?;

        leistung_from_row(&row)
    }

    async fn find_offene_by_datum(
        &self,
        datum: NaiveDate,
    ) -> RepositoryResult<Vec<Versioned<LeistungOffen>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let datum_str = format_naive_date(datum);
        let mut rows = tx
            .query(
                &format!("{LEISTUNG_SELECT} WHERE rechnung_id IS NULL AND leistungsdatum = ?1"),
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
            leistungen.push(leistung_offen_from_row(&row)?);
        }
        Ok(leistungen)
    }

    async fn update(&self, leistung: &mut Versioned<Leistung>) -> RepositoryResult<()> {
        let id_str = leistung.id().0.to_string();
        let version = leistung.v();

        let (status, rechnung_id_str) = match &**leistung {
            Leistung::Offen(_) => ("offen", None),
            Leistung::Abgerechnet(abgerechnet) => {
                ("abgerechnet", Some(abgerechnet.rechnung_id().0.to_string()))
            }
        };

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "UPDATE leistungen SET status = ?1, rechnung_id = ?2, _version = _version + 1 WHERE id = ?3 AND _version = ?4",
                libsql::params![status, rechnung_id_str, id_str, version],
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
        Ok(())
    }
}
