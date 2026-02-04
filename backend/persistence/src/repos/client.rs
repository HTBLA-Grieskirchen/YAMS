use std::{str::FromStr, sync::Arc};

use async_lock::Mutex;
use async_trait::async_trait;
use chrono::NaiveDate;
use libsql::Transaction;
use uuid::Uuid;
use yams_core::{
    domain::{Address, Client, ClientId, Email, MobileNumber, factories::NewClient},
    ports::repos::{ClientRepository, RepositoryResult, Versioned},
};

use crate::errors::ToPersistenceResultExt;

pub struct SQLiteClientRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn parse_naive_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

#[async_trait]
impl ClientRepository for SQLiteClientRepository {
    async fn find_by_id(&self, id: ClientId) -> RepositoryResult<Option<Versioned<Client>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or(yams_core::service::errors::PersistenceError::ConcurrentModification)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, first_name, last_name, birthdate, email, mobile_number, customer_number, consent, postal_code, city, street_and_number, country_code, _version FROM clients WHERE id = ?1",
                [id_str.clone()],
            )
            .await
            .to_persistence()?;

        let Some(row) = rows.next().await.to_persistence()? else {
            return Ok(None);
        };

        let id_raw: String = row.get(0).to_persistence()?;
        let first_name: String = row.get(1).to_persistence()?;
        let last_name: String = row.get(2).to_persistence()?;
        let birthdate_str: String = row.get(3).to_persistence()?;
        let email_str: String = row.get(4).to_persistence()?;
        let mobile_number_str: String = row.get(5).to_persistence()?;
        let customer_number: i64 = row.get(6).to_persistence()?;
        let consent: bool = row.get::<i64>(7).to_persistence()? != 0;
        let postal_code: String = row.get(8).to_persistence()?;
        let city: String = row.get(9).to_persistence()?;
        let street_and_number: String = row.get(10).to_persistence()?;
        let country_code: String = row.get(11).to_persistence()?;
        let version: u64 = row.get(12).to_persistence()?;

        let birthdate = parse_naive_date(&birthdate_str).map_err(|e| {
            yams_core::service::errors::PersistenceError::DeserializationError(anyhow::anyhow!(e))
        })?;
        let uuid = Uuid::from_str(&id_raw).map_err(|e| {
            yams_core::service::errors::PersistenceError::DeserializationError(anyhow::anyhow!(e))
        })?;

        let mut animal_rows = tx
            .query("SELECT id FROM animals WHERE client_id = ?1", [id_str])
            .await
            .to_persistence()?;
        let mut animal_ids = Vec::new();
        while let Some(arow) = animal_rows.next().await.to_persistence()? {
            let aid: String = arow.get(0).to_persistence()?;
            let auuid = Uuid::from_str(&aid).map_err(|e| {
                yams_core::service::errors::PersistenceError::DeserializationError(anyhow::anyhow!(
                    e
                ))
            })?;
            animal_ids.push(yams_core::domain::AnimalId(auuid));
        }

        let client = Client {
            id: ClientId(uuid),
            first_name,
            last_name,
            birthdate,
            email: Email(email_str),
            mobile_number: MobileNumber(mobile_number_str),
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
        Ok(Some(Versioned::new(version, client)))
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
        let tx = guard
            .as_mut()
            .ok_or(yams_core::service::errors::PersistenceError::ConcurrentModification)?;

        tx.execute(
            "INSERT INTO clients (id, first_name, last_name, birthdate, email, mobile_number, customer_number, consent, postal_code, city, street_and_number, country_code, _version) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            libsql::params![
                id_str,
                client.first_name.clone(),
                client.last_name.clone(),
                birthdate_str,
                client.email.0.clone(),
                client.mobile_number.0.clone(),
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
        .to_persistence()?;

        Ok(client)
    }

    async fn update(&self, client: &mut Versioned<Client>) -> RepositoryResult<()> {
        let id_str = client.id.0.to_string();
        let birthdate_str = client.birthdate.format("%Y-%m-%d").to_string();
        let consent_i: i64 = if client.consent { 1 } else { 0 };
        let version = client.v();

        let mut guard = self.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or(yams_core::service::errors::PersistenceError::ConcurrentModification)?;

        let result = tx
            .execute(
                "UPDATE clients SET first_name = ?1, last_name = ?2, birthdate = ?3, email = ?4, mobile_number = ?5, customer_number = ?6, consent = ?7, postal_code = ?8, city = ?9, street_and_number = ?10, country_code = ?11, _version = _version + 1 WHERE id = ?12 AND _version = ?13",
                libsql::params![
                    client.first_name.clone(),
                    client.last_name.clone(),
                    birthdate_str,
                    client.email.0.clone(),
                    client.mobile_number.0.clone(),
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
            .to_persistence()?;

        if result != 1 {
            return Err(
                yams_core::service::errors::PersistenceError::VersionMismatch {
                    expected: version,
                    actual: None,
                },
            );
        }

        tx.execute(
            "UPDATE animals SET client_id = NULL WHERE client_id = ?1",
            [id_str.clone()],
        )
        .await
        .to_persistence()?;

        for animal_id in &client.animal_ids {
            tx.execute(
                "UPDATE animals SET client_id = ?1 WHERE id = ?2",
                libsql::params![id_str.clone(), animal_id.0.to_string()],
            )
            .await
            .to_persistence()?;
        }

        *client = client.clone().incremented();
        Ok(())
    }

    async fn delete(&self, client: Versioned<Client>) -> RepositoryResult<()> {
        let id_str = client.id.0.to_string();
        let version = client.v();

        let mut guard = self.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or(yams_core::service::errors::PersistenceError::ConcurrentModification)?;

        tx.execute(
            "UPDATE animals SET client_id = NULL WHERE client_id = ?1",
            [id_str.clone()],
        )
        .await
        .to_persistence()?;

        let result = tx
            .execute(
                "DELETE FROM clients WHERE id = ?1 AND _version = ?2",
                libsql::params![id_str, version],
            )
            .await
            .to_persistence()?;

        if result != 1 {
            return Err(
                yams_core::service::errors::PersistenceError::VersionMismatch {
                    expected: version,
                    actual: None,
                },
            );
        }
        Ok(())
    }
}
