use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::{Row, Transaction};
use uuid::Uuid;
use yams_core::{
    ErrorReportExt,
    domain::{KlientId, LeistungId, Rechnung, RechnungId, RechnungOffen, Rechnungsposition},
    ports::{RechnungRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{
    decimal_to_str, format_naive_date, parse_decimal, parse_klient_id, parse_naive_date,
    parse_preis, parse_uuid, preis_to_str,
};

pub struct SQLiteRechnungRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

struct RechnungRowData {
    id: RechnungId,
    rechnungsnummer: u64,
    klient_id: KlientId,
    rechnungsdatum: chrono::NaiveDate,
    status: String,
    bezahlt_datum: Option<chrono::NaiveDate>,
    version: u64,
}

fn parse_rechnung_header(row: &Row) -> RepositoryResult<RechnungRowData> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let rechnungsnummer: i64 = row.get(1).contextualize(RepositoryError::Data)?;
    let klient_id_str: String = row.get(2).contextualize(RepositoryError::Data)?;
    let rechnungsdatum_str: String = row.get(3).contextualize(RepositoryError::Data)?;
    let status: String = row.get(4).contextualize(RepositoryError::Data)?;
    let bezahlt_datum_str: Option<String> = row.get(5).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(6).contextualize(RepositoryError::Data)?;

    let uuid = parse_uuid(&id_raw)?;
    let klient_id = parse_klient_id(&klient_id_str)?;
    let rechnungsdatum =
        parse_naive_date(&rechnungsdatum_str).contextualize(RepositoryError::Data)?;
    let bezahlt_datum = bezahlt_datum_str
        .map(|s| parse_naive_date(&s))
        .transpose()
        .contextualize(RepositoryError::Data)?;

    Ok(RechnungRowData {
        id: RechnungId(uuid),
        rechnungsnummer: rechnungsnummer as u64,
        klient_id,
        rechnungsdatum,
        status,
        bezahlt_datum,
        version,
    })
}

fn parse_position_from_row(row: &Row) -> RepositoryResult<Rechnungsposition> {
    let leistung_id_str: Option<String> = row.get(7).contextualize(RepositoryError::Data)?;
    let beschreibung: Option<String> = row.get(8).contextualize(RepositoryError::Data)?;
    let einzelpreis_str: Option<String> = row.get(9).contextualize(RepositoryError::Data)?;
    let stückzahl_str: Option<String> = row.get(10).contextualize(RepositoryError::Data)?;
    let mwst_str: Option<String> = row.get(11).contextualize(RepositoryError::Data)?;

    let leistung_id_str = leistung_id_str.ok_or(RepositoryError::Data)?;
    let beschreibung = beschreibung.ok_or(RepositoryError::Data)?;
    let einzelpreis_str = einzelpreis_str.ok_or(RepositoryError::Data)?;
    let stückzahl_str = stückzahl_str.ok_or(RepositoryError::Data)?;
    let mwst_str = mwst_str.ok_or(RepositoryError::Data)?;

    let leistung_uuid = parse_uuid(&leistung_id_str)?;
    Ok(Rechnungsposition::neu(
        beschreibung,
        parse_preis(&einzelpreis_str)?,
        parse_decimal(&stückzahl_str)?,
        parse_decimal(&mwst_str)?,
        LeistungId(leistung_uuid),
    ))
}

fn geladene_rechnung_from_parts(
    header: &RechnungRowData,
    positionen: Vec<Rechnungsposition>,
) -> RepositoryResult<Rechnung> {
    let bezahlt_datum = if header.status == "bezahlt" {
        Some(header.bezahlt_datum.ok_or(RepositoryError::Data)?)
    } else {
        None
    };

    Ok(Rechnung::from_parts(
        header.id.clone(),
        header.rechnungsnummer,
        header.klient_id.clone(),
        header.rechnungsdatum,
        positionen,
        bezahlt_datum,
    )
    .map_err(|_| RepositoryError::Data)?)
}

#[async_trait]
impl RechnungRepository for SQLiteRechnungRepository {
    async fn create(&self, rechnung: RechnungOffen) -> RepositoryResult<Versioned<RechnungOffen>> {
        let versioned = Versioned::init(rechnung.clone());
        let gesamtbetrag = preis_to_str(&rechnung.gesamtbetrag_brutto());

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO rechnungen (id, rechnungsnummer, klient_id, rechnungsdatum, gesamtbetrag, status, _version) VALUES (?1, ?2, ?3, ?4, ?5, 'offen', ?6)",
            libsql::params![
                rechnung.id().0.to_string(),
                rechnung.rechnungsnummer(),
                rechnung.klient_id().0.to_string(),
                format_naive_date(rechnung.rechnungsdatum()),
                gesamtbetrag,
                versioned.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        for position in rechnung.positionen() {
            tx.execute(
                "INSERT INTO rechnungspositionen (id, rechnung_id, leistung_id, beschreibung, einzelpreis, \"stückzahl\", mwst_prozentsatz) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                libsql::params![
                    Uuid::new_v4().to_string(),
                    rechnung.id().0.to_string(),
                    position.leistung_id().0.to_string(),
                    position.beschreibung(),
                    preis_to_str(position.einzelpreis()),
                    decimal_to_str(&position.stückzahl()),
                    decimal_to_str(&position.mwst_prozentsatz()),
                ],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        }

        Ok(versioned)
    }

    async fn nächste_rechnungsnummer(&self) -> RepositoryResult<u64> {
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
        Ok(nummer as u64)
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
                "SELECT r.id, r.rechnungsnummer, r.klient_id, r.rechnungsdatum, r.status, r.bezahlt_datum, r._version, p.leistung_id, p.beschreibung, p.einzelpreis, p.\"stückzahl\", p.mwst_prozentsatz FROM rechnungen r LEFT JOIN rechnungspositionen p ON p.rechnung_id = r.id WHERE r.klient_id = ?1 ORDER BY r.rechnungsnummer, p.id",
                [klient_id_str],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut rechnungen: Vec<Versioned<Rechnung>> = Vec::new();
        let mut current_header: Option<RechnungRowData> = None;
        let mut current_positionen: Vec<Rechnungsposition> = Vec::new();

        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            let header = parse_rechnung_header(&row)?;
            let is_new_rechnung = current_header.as_ref().is_none_or(|h| h.id != header.id);

            if is_new_rechnung {
                if let Some(prev_header) = current_header {
                    let version = prev_header.version;
                    let geladen = geladene_rechnung_from_parts(&prev_header, current_positionen)?;
                    rechnungen.push(Versioned::new(version, geladen));
                    current_positionen = Vec::new();
                }
                current_header = Some(header);
            }

            if let Ok(position) = parse_position_from_row(&row) {
                current_positionen.push(position);
            }
        }

        if let Some(header) = current_header {
            let version = header.version;
            let geladen = geladene_rechnung_from_parts(&header, current_positionen)?;
            rechnungen.push(Versioned::new(version, geladen));
        }

        Ok(rechnungen)
    }
}
