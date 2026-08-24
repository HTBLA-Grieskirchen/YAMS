use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use error_stack::ResultExt;
use libsql::{Row, Transaction};
use uuid::Uuid;
use yams_core::{
    ErrorReportExt,
    domain::{Adresse, Klient, KlientId, Ländercode, klient::NeuerKlient},
    ports::{KlientRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
};

use crate::errors::libsql_error_to_persistence_error;

use super::common::{format_naive_date, parse_naive_date, parse_uuid};

pub struct SQLiteKlientRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn klient_from_row(row: &Row) -> RepositoryResult<Versioned<Klient>> {
    let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
    let vorname: String = row.get(1).contextualize(RepositoryError::Data)?;
    let nachname: String = row.get(2).contextualize(RepositoryError::Data)?;
    let geburtstag_str: String = row.get(3).contextualize(RepositoryError::Data)?;
    let email_str: String = row.get(4).contextualize(RepositoryError::Data)?;
    let mobilnummer_str: String = row.get(5).contextualize(RepositoryError::Data)?;
    let kundennummer: i64 = row.get(6).contextualize(RepositoryError::Data)?;
    let einwilligung: bool = row.get::<i64>(7).contextualize(RepositoryError::Data)? != 0;
    let postleitzahl: String = row.get(8).contextualize(RepositoryError::Data)?;
    let stadt: String = row.get(9).contextualize(RepositoryError::Data)?;
    let straße_und_hausnummer: String = row.get(10).contextualize(RepositoryError::Data)?;
    let ländercode_str: String = row.get(11).contextualize(RepositoryError::Data)?;
    let version: u64 = row.get(12).contextualize(RepositoryError::Data)?;

    let geburtstag = parse_naive_date(&geburtstag_str).contextualize(RepositoryError::Data)?;
    let uuid = parse_uuid(&id_raw).contextualize(RepositoryError::Data)?;

    let klient = Klient::from_parts(
        KlientId(uuid),
        vorname,
        nachname,
        geburtstag,
        email_str,
        mobilnummer_str,
        kundennummer as u64,
        einwilligung,
        Adresse {
            postleitzahl,
            stadt,
            straße_und_hausnummer,
            ländercode: Ländercode::from_str(&ländercode_str)
                .change_context(RepositoryError::Data)?,
        },
    )
    .change_context(RepositoryError::Data)?;
    Ok(Versioned::new(version, klient))
}

#[async_trait]
impl KlientRepository for SQLiteKlientRepository {
    async fn find_by_id(&self, id: KlientId) -> RepositoryResult<Versioned<Klient>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, vorname, nachname, geburtstag, email, mobilnummer, kundennummer, einwilligung, postleitzahl, stadt, \"straße_und_hausnummer\", \"ländercode\", _version FROM klienten WHERE id = ?1",
                [id_str],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::NotFound)?;

        klient_from_row(&row)
    }

    async fn create(&self, new: NeuerKlient) -> RepositoryResult<Versioned<Klient>> {
        let id = KlientId(Uuid::new_v4());
        let klient = Versioned::init(Klient::neu(id, new).change_context(RepositoryError::Data)?);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO klienten (id, vorname, nachname, geburtstag, email, mobilnummer, kundennummer, einwilligung, postleitzahl, stadt, \"straße_und_hausnummer\", \"ländercode\", _version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            libsql::params![
                klient.id().0.to_string(),
                klient.vorname(),
                klient.nachname(),
                format_naive_date(klient.geburtstag()),
                klient.email().as_ref(),
                klient.mobilnummer().as_ref(),
                klient.kundennummer(),
                if klient.einwilligung() { 1i64 } else { 0i64 },
                klient.adresse().postleitzahl.clone(),
                klient.adresse().stadt.clone(),
                klient.adresse().straße_und_hausnummer.clone(),
                klient.adresse().ländercode.as_str(),
                klient.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(klient)
    }

    async fn update(&self, klient: &mut Versioned<Klient>) -> RepositoryResult<()> {
        let id_str = klient.id().0.to_string();
        let version = klient.v();

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "UPDATE klienten SET vorname = ?1, nachname = ?2, geburtstag = ?3, email = ?4, mobilnummer = ?5, kundennummer = ?6, einwilligung = ?7, postleitzahl = ?8, stadt = ?9, \"straße_und_hausnummer\" = ?10, \"ländercode\" = ?11, _version = _version + 1 WHERE id = ?12 AND _version = ?13",
                libsql::params![
                    klient.vorname(),
                    klient.nachname(),
                    format_naive_date(klient.geburtstag()),
                    klient.email().as_ref(),
                    klient.mobilnummer().as_ref(),
                    klient.kundennummer(),
                    if klient.einwilligung() { 1i64 } else { 0i64 },
                    klient.adresse().postleitzahl.clone(),
                    klient.adresse().stadt.clone(),
                    klient.adresse().straße_und_hausnummer.clone(),
                    klient.adresse().ländercode.as_str(),
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

        *klient = klient.clone().incremented();
        Ok(())
    }

    async fn delete(&self, klient: Versioned<Klient>) -> RepositoryResult<()> {
        let id_str = klient.id().0.to_string();
        let version = klient.v();

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "DELETE FROM klienten WHERE id = ?1 AND _version = ?2",
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
