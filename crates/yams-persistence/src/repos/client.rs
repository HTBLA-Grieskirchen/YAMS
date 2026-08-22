use std::{str::FromStr, sync::Arc};

use async_lock::Mutex;
use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{IntoReport, ResultExt};
use libsql::Transaction;
use uuid::Uuid;
use yams_core::{
    ErrorReportExt,
    domain::{Address, Client, ClientId, EmailAddress, MobileNumber, client::NewClient},
    ports::{ClientRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
};

use crate::errors::libsql_error_to_persistence_error;

pub struct SQLiteClientRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn parse_naive_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

#[async_trait]
impl ClientRepository for SQLiteClientRepository {
    async fn find_by_id(&self, id: ClientId) -> RepositoryResult<Versioned<Client>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, first_name, last_name, birthdate, email, mobile_number, customer_number, consent, postal_code, city, street_and_number, country_code, _version FROM clients WHERE id = ?1",
                [id_str.clone()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        let row = rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
            .ok_or(RepositoryError::NotFound)?;

        let id_raw: String = row
            .get(0)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let first_name: String = row
            .get(1)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let last_name: String = row
            .get(2)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let birthdate_str: String = row
            .get(3)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let email_str: String = row
            .get(4)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let mobile_number_str: String = row
            .get(5)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let customer_number: i64 = row
            .get(6)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let consent: bool = row
            .get::<i64>(7)
            .contextualize_with(libsql_error_to_persistence_error)?
            != 0;
        let postal_code: String = row
            .get(8)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let city: String = row
            .get(9)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let street_and_number: String = row
            .get(10)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let country_code: String = row
            .get(11)
            .contextualize_with(libsql_error_to_persistence_error)?;
        let version: u64 = row
            .get(12)
            .contextualize_with(libsql_error_to_persistence_error)?;

        let birthdate = parse_naive_date(&birthdate_str).contextualize(RepositoryError::Data)?;
        let uuid = Uuid::from_str(&id_raw).contextualize(RepositoryError::Data)?;

        let mut animal_rows = tx
            .query("SELECT id FROM animals WHERE client_id = ?1", [id_str])
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        let mut animal_ids = Vec::new();
        while let Some(arow) = animal_rows
            .next()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?
        {
            let aid: String = arow
                .get(0)
                .contextualize_with(libsql_error_to_persistence_error)?;
            let auuid = Uuid::from_str(&aid)
                .map_err(IntoReport::into_report)
                .attach(format!("animal uuid {aid}"))
                .change_context(RepositoryError::Data)?;
            animal_ids.push(yams_core::domain::AnimalId(auuid));
        }

        let client = Client {
            id: ClientId(uuid),
            first_name,
            last_name,
            birthdate,
            email: EmailAddress::new(email_str).change_context(RepositoryError::Data)?,
            mobile_number: MobileNumber::new(mobile_number_str)
                .change_context(RepositoryError::Data)?,
            customer_number,
            consent,
            address: Address {
                postal_code,
                city,
                street_and_number,
                country_code,
            },
            animal_ids,
        };
        Ok(Versioned::new(version, client))
    }

    async fn create(&self, new: NewClient) -> RepositoryResult<Versioned<Client>> {
        let id = ClientId(Uuid::new_v4());
        let id_str = id.0.to_string();
        let birthdate_str = new.birthdate.format("%Y-%m-%d").to_string();
        let consent_i: i64 = if new.consent { 1 } else { 0 };
        let client = Client {
            id,
            first_name: new.first_name,
            last_name: new.last_name,
            birthdate: new.birthdate,
            email: new.email,
            mobile_number: new.mobile_number,
            customer_number: new.customer_number,
            consent: new.consent,
            address: new.address,
            animal_ids: Vec::new(),
        };
        let client = Versioned::init(client);

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO clients (id, first_name, last_name, birthdate, email, mobile_number, customer_number, consent, postal_code, city, street_and_number, country_code, _version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            libsql::params![
                id_str,
                client.first_name.clone(),
                client.last_name.clone(),
                birthdate_str,
                client.email.as_ref(),
                client.mobile_number.as_ref(),
                client.customer_number,
                consent_i,
                client.address.postal_code.clone(),
                client.address.city.clone(),
                client.address.street_and_number.clone(),
                client.address.country_code.clone(),
                client.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(client)
    }

    async fn update(&self, client: &mut Versioned<Client>) -> RepositoryResult<()> {
        let id_str = client.id.0.to_string();
        let birthdate_str = client.birthdate.format("%Y-%m-%d").to_string();
        let consent_i: i64 = if client.consent { 1 } else { 0 };
        let version = client.v();

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "UPDATE clients SET first_name = ?1, last_name = ?2, birthdate = ?3, email = ?4, mobile_number = ?5, customer_number = ?6, consent = ?7, postal_code = ?8, city = ?9, street_and_number = ?10, country_code = ?11, _version = _version + 1 WHERE id = ?12 AND _version = ?13",
                libsql::params![
                    client.first_name.clone(),
                    client.last_name.clone(),
                    birthdate_str,
                    client.email.as_ref(),
                    client.mobile_number.as_ref(),
                    client.customer_number,
                    consent_i,
                    client.address.postal_code.clone(),
                    client.address.city.clone(),
                    client.address.street_and_number.clone(),
                    client.address.country_code.clone(),
                    id_str.clone(),
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

        tx.execute(
            "UPDATE animals SET client_id = NULL WHERE client_id = ?1",
            [id_str.clone()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        for animal_id in &client.animal_ids {
            tx.execute(
                "UPDATE animals SET client_id = ?1 WHERE id = ?2",
                libsql::params![id_str.clone(), animal_id.0.to_string()],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        }

        *client = client.clone().incremented();
        Ok(())
    }

    async fn delete(&self, client: Versioned<Client>) -> RepositoryResult<()> {
        let id_str = client.id.0.to_string();
        let version = client.v();

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "UPDATE animals SET client_id = NULL WHERE client_id = ?1",
            [id_str.clone()],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        let result = tx
            .execute(
                "DELETE FROM clients WHERE id = ?1 AND _version = ?2",
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
