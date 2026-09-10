use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use rustc_hash::FxHashMap;
use uuid::Uuid;
use yams_core::{
    ErrorReportExt,
    domain::{
        Adresse, KlientId, LeistungId, Ländercode, SeminarBuchung, SeminarBuchungId, SeminarId,
        SeminarOrt, SeminarTermin, SeminarTerminGeplant, SeminarTerminId,
        SeminarTerminZustandTeile, Zeitraum, seminar_termin::NeuerSeminarTermin,
    },
    ports::{RepositoryError, RepositoryResult, SeminarTerminRepository},
    uow::Versioned,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{format_datetime, parse_datetime, parse_ratio, parse_uuid, ratio_to_str};

const TERMIN_SELECT: &str = "SELECT id, seminar_id, beginn, ende, ort_name, postleitzahl, stadt, \"straße_und_hausnummer\", \"ländercode\", max_teilnehmer, status, abgehalten_am, abgesagt_am, absagegrund, _version FROM seminar_termine";

pub struct SQLiteSeminarTerminRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn ort_from_columns(
    ort_name: Option<String>,
    postleitzahl: Option<String>,
    stadt: Option<String>,
    straße: Option<String>,
    ländercode: Option<String>,
) -> RepositoryResult<SeminarOrt> {
    let adresse = match (postleitzahl, stadt, straße, ländercode) {
        (Some(postleitzahl), Some(stadt), Some(straße_und_hausnummer), Some(code)) => {
            Some(Adresse {
                postleitzahl,
                stadt,
                straße_und_hausnummer,
                ländercode: Ländercode::from_str(&code).map_err(|_| RepositoryError::Data)?,
            })
        }
        (None, None, None, None) => None,
        _ => return Err(RepositoryError::Data)?,
    };
    Ok(SeminarOrt::neu(ort_name, adresse))
}

fn zustand_from_row(
    status: &str,
    abgehalten_am: Option<String>,
    abgesagt_am: Option<String>,
    absagegrund: Option<String>,
    leistungen: FxHashMap<SeminarBuchungId, LeistungId>,
) -> RepositoryResult<SeminarTerminZustandTeile> {
    match status {
        "geplant" => Ok(SeminarTerminZustandTeile::Geplant),
        "abgehalten" => {
            let abgehalten_am = parse_datetime(&abgehalten_am.ok_or(RepositoryError::Data)?)?;
            Ok(SeminarTerminZustandTeile::Abgehalten {
                abgehalten_am,
                leistungen,
            })
        }
        "abgesagt" => {
            let abgesagt_am = parse_datetime(&abgesagt_am.ok_or(RepositoryError::Data)?)?;
            Ok(SeminarTerminZustandTeile::Abgesagt {
                abgesagt_am,
                grund: absagegrund.unwrap_or_default(),
            })
        }
        _ => Err(RepositoryError::Data)?,
    }
}

fn status_columns(
    termin: &SeminarTermin,
) -> (&'static str, Option<String>, Option<String>, Option<String>) {
    match termin {
        SeminarTermin::Geplant(_) => ("geplant", None, None, None),
        SeminarTermin::Abgehalten(t) => (
            "abgehalten",
            Some(format_datetime(t.abgehalten_am())),
            None,
            None,
        ),
        SeminarTermin::Abgesagt(t) => (
            "abgesagt",
            None,
            Some(format_datetime(t.abgesagt_am())),
            Some(t.grund().to_string()),
        ),
    }
}

fn leistung_id_for(termin: &SeminarTermin, buchung: &SeminarBuchung) -> Option<String> {
    match termin {
        SeminarTermin::Abgehalten(abgehalten) => abgehalten
            .leistung_fuer_buchung(buchung.id())
            .map(|id| id.0.to_string()),
        _ => None,
    }
}

fn termin_from_parts(
    row: &Row,
    buchungen: Vec<SeminarBuchung>,
    leistungen: FxHashMap<SeminarBuchungId, LeistungId>,
) -> RepositoryResult<Versioned<SeminarTermin>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let seminar_id_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
    let beginn_str: String = row.get(2).contextualize(RepositoryError::Data)?;
    let ende_str: String = row.get(3).contextualize(RepositoryError::Data)?;
    let ort_name: Option<String> = row.get(4).contextualize(RepositoryError::Data)?;
    let postleitzahl: Option<String> = row.get(5).contextualize(RepositoryError::Data)?;
    let stadt: Option<String> = row.get(6).contextualize(RepositoryError::Data)?;
    let straße: Option<String> = row.get(7).contextualize(RepositoryError::Data)?;
    let ländercode: Option<String> = row.get(8).contextualize(RepositoryError::Data)?;
    let max_teilnehmer: Option<i64> = row.get(9).contextualize(RepositoryError::Data)?;
    let status: String = row.get(10).contextualize(RepositoryError::Data)?;
    let abgehalten_am: Option<String> = row.get(11).contextualize(RepositoryError::Data)?;
    let abgesagt_am: Option<String> = row.get(12).contextualize(RepositoryError::Data)?;
    let absagegrund: Option<String> = row.get(13).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(14).contextualize(RepositoryError::Data)?;

    let zeitraum = Zeitraum::neu(parse_datetime(&beginn_str)?, parse_datetime(&ende_str)?)
        .change_context(RepositoryError::Data)?;
    let ort = ort_from_columns(ort_name, postleitzahl, stadt, straße, ländercode)?;
    let zustand = zustand_from_row(&status, abgehalten_am, abgesagt_am, absagegrund, leistungen)?;

    Ok(Versioned::new(
        version,
        SeminarTermin::from_parts(
            SeminarTerminId(parse_uuid(&id_raw)?),
            SeminarId(parse_uuid(&seminar_id_raw)?),
            zeitraum,
            ort,
            max_teilnehmer.map(|n| n as u32),
            buchungen,
            zustand,
        ),
    ))
}

async fn load_buchungen(
    tx: &Transaction,
    termin_id: &str,
) -> RepositoryResult<(Vec<SeminarBuchung>, FxHashMap<SeminarBuchungId, LeistungId>)> {
    let mut rows = tx
        .query(
            "SELECT id, klient_id, rabatt, storniert_am, leistung_id FROM seminar_buchungen WHERE termin_id = ?1",
            [termin_id],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

    let mut buchungen = Vec::new();
    let mut leistungen = FxHashMap::default();
    while let Some(row) = rows
        .next()
        .await
        .contextualize_with(libsql_error_to_persistence_error)?
    {
        let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
        let klient_id_raw: String = row.get(1).contextualize(RepositoryError::Data)?;
        let rabatt_str: String = row.get(2).contextualize(RepositoryError::Data)?;
        let storniert_am: Option<String> = row.get(3).contextualize(RepositoryError::Data)?;
        let leistung_id_raw: Option<String> = row.get(4).contextualize(RepositoryError::Data)?;

        let buchung_id = SeminarBuchungId(parse_uuid(&id_raw)?);
        let buchung = SeminarBuchung::from_parts(
            buchung_id.clone(),
            KlientId(parse_uuid(&klient_id_raw)?),
            parse_ratio(&rabatt_str)?,
            storniert_am.map(|s| parse_datetime(&s)).transpose()?,
        );
        if let Some(leistung_id_raw) = leistung_id_raw {
            leistungen.insert(buchung_id, LeistungId(parse_uuid(&leistung_id_raw)?));
        }
        buchungen.push(buchung);
    }
    Ok((buchungen, leistungen))
}

async fn load_termin(tx: &Transaction, row: &Row) -> RepositoryResult<Versioned<SeminarTermin>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let (buchungen, leistungen) = load_buchungen(tx, &id_raw).await?;
    termin_from_parts(row, buchungen, leistungen)
}

async fn replace_buchungen(tx: &Transaction, termin: &SeminarTermin) -> RepositoryResult<()> {
    let termin_id = termin.id().0.to_string();
    tx.execute(
        "DELETE FROM seminar_buchungen WHERE termin_id = ?1",
        [termin_id.clone()],
    )
    .await
    .contextualize_with(libsql_error_to_persistence_error)?;

    for buchung in termin.buchungen() {
        tx.execute(
            "INSERT INTO seminar_buchungen (id, termin_id, klient_id, rabatt, storniert_am, leistung_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            libsql::params![
                buchung.id().0.to_string(),
                termin_id.clone(),
                buchung.klient_id().0.to_string(),
                ratio_to_str(buchung.rabatt()),
                buchung.storniert_am().map(format_datetime),
                leistung_id_for(termin, buchung),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;
    }
    Ok(())
}

fn insert_params(termin: &SeminarTermin, version: u64) -> impl libsql::params::IntoParams {
    let ort = termin.ort();
    let adresse = ort.adresse();
    let (status, abgehalten_am, abgesagt_am, absagegrund) = status_columns(termin);
    libsql::params![
        termin.id().0.to_string(),
        termin.seminar_id().0.to_string(),
        format_datetime(termin.zeitraum().beginn()),
        format_datetime(termin.zeitraum().ende()),
        ort.ort_name().map(str::to_string),
        adresse.map(|a| a.postleitzahl.clone()),
        adresse.map(|a| a.stadt.clone()),
        adresse.map(|a| a.straße_und_hausnummer.clone()),
        adresse.map(|a| a.ländercode.as_str().to_string()),
        termin.max_teilnehmer().map(|n| n as i64),
        status,
        abgehalten_am,
        abgesagt_am,
        absagegrund,
        version,
    ]
}

#[async_trait]
impl SeminarTerminRepository for SQLiteSeminarTerminRepository {
    async fn find_by_id(&self, id: SeminarTerminId) -> RepositoryResult<Versioned<SeminarTermin>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;
        let id_str = id.0.to_string();
        let mut rows = tx
            .query(&format!("{TERMIN_SELECT} WHERE id = ?1"), [id_str])
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::NotFound)?;
        load_termin(tx, &row).await
    }

    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<SeminarTermin>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;
        let mut rows = tx
            .query(&format!("{TERMIN_SELECT} ORDER BY beginn"), ())
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut termine = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            termine.push(load_termin(tx, &row).await?);
        }
        Ok(termine)
    }

    async fn find_by_seminar_id(
        &self,
        seminar_id: SeminarId,
    ) -> RepositoryResult<Vec<Versioned<SeminarTermin>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;
        let mut rows = tx
            .query(
                &format!("{TERMIN_SELECT} WHERE seminar_id = ?1"),
                [seminar_id.0.to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut termine = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            termine.push(load_termin(tx, &row).await?);
        }
        Ok(termine)
    }

    async fn find_nicht_vollständig_abgerechnet_bis(
        &self,
        stichtag: NaiveDate,
    ) -> RepositoryResult<Vec<Versioned<SeminarTermin>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;
        let mut rows = tx
            .query(
                &format!("{TERMIN_SELECT} WHERE status IN ('geplant', 'abgehalten')"),
                (),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let mut termine = Vec::new();
        while let Some(row) = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            let termin = load_termin(tx, &row).await?;
            if termin.zeitraum().ende().date_naive() > stichtag {
                continue;
            }
            match &*termin {
                SeminarTermin::Geplant(_) => termine.push(termin),
                SeminarTermin::Abgehalten(abgehalten) => {
                    let mut offen = false;
                    for leistung_id in abgehalten.leistungen().values() {
                        let mut check = tx
                            .query(
                                "SELECT 1 FROM leistungen WHERE id = ?1 AND rechnung_id IS NULL",
                                [leistung_id.0.to_string()],
                            )
                            .await
                            .contextualize_with(libsql_error_to_persistence_error)?;
                        if check
                            .next()
                            .await
                            .contextualize_with(libsql_error_to_persistence_error)?
                            .is_some()
                        {
                            offen = true;
                            break;
                        }
                    }
                    if offen {
                        termine.push(termin);
                    }
                }
                SeminarTermin::Abgesagt(_) => {}
            }
        }
        Ok(termine)
    }

    async fn create(
        &self,
        neu: NeuerSeminarTermin,
    ) -> RepositoryResult<Versioned<SeminarTerminGeplant>> {
        let geplant = SeminarTerminGeplant::neu(SeminarTerminId(Uuid::new_v4()), neu);
        let versioned = Versioned::init(SeminarTermin::from(geplant.clone()));

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;
        tx.execute(
            "INSERT INTO seminar_termine (id, seminar_id, beginn, ende, ort_name, postleitzahl, stadt, \"straße_und_hausnummer\", \"ländercode\", max_teilnehmer, status, abgehalten_am, abgesagt_am, absagegrund, _version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            insert_params(&versioned, versioned.v()),
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(Versioned::init(geplant))
    }

    async fn update(&self, termin: &mut Versioned<SeminarTermin>) -> RepositoryResult<()> {
        let version = termin.v();
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "UPDATE seminar_termine SET seminar_id = ?2, beginn = ?3, ende = ?4, ort_name = ?5, postleitzahl = ?6, stadt = ?7, \"straße_und_hausnummer\" = ?8, \"ländercode\" = ?9, max_teilnehmer = ?10, status = ?11, abgehalten_am = ?12, abgesagt_am = ?13, absagegrund = ?14, _version = _version + 1 WHERE id = ?1 AND _version = ?15",
                insert_params(termin, version),
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if result != 1 {
            Err(RepositoryError::VersionMismatch {
                expected: version,
                actual: None,
            })?;
        }

        replace_buchungen(tx, termin).await?;
        termin.increment();
        Ok(())
    }
}
