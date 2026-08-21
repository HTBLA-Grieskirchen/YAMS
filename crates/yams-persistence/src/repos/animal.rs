use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::bail;
use uuid::Uuid;
use yams_core::{
    ErrorReportExt,
    domain::{Animal, AnimalId, factories::NewAnimal},
    ports::{AnimalRepository, RepositoryError, RepositoryResult},
    uow::Versioned,
};

use crate::errors::libsql_error_to_persistence_error;
use async_lock::Mutex;
use libsql::Transaction;

pub struct SQLiteAnimalRepository {
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
}

fn parse_naive_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

#[async_trait]
impl AnimalRepository for SQLiteAnimalRepository {
    async fn find_by_id(&self, id: AnimalId) -> RepositoryResult<Versioned<Animal>> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, name, birthdate, animal_species, description, _version FROM animals WHERE id = ?1",
                [id_str],
            )
            .await
            .contextualize(RepositoryError::NotFound)?;

        let row = rows
            .next()
            .await
            .contextualize(RepositoryError::NotFound)?
            .ok_or(RepositoryError::NotFound)?;

        let id_raw: String = row.get(0).contextualize(RepositoryError::Data)?;
        let name: String = row.get(1).contextualize(RepositoryError::Data)?;
        let birthdate_str: String = row.get(2).contextualize(RepositoryError::Data)?;
        let animal_species: String = row.get(3).contextualize(RepositoryError::Data)?;
        let description: String = row.get(4).contextualize(RepositoryError::Data)?;
        let version: u64 = row.get(5).contextualize(RepositoryError::Data)?;

        let birthdate = parse_naive_date(&birthdate_str).contextualize(RepositoryError::Data)?;
        let uuid = Uuid::from_str(&id_raw).contextualize(RepositoryError::Data)?;

        let animal = Animal {
            id: AnimalId(uuid),
            name,
            birthdate,
            animal_species,
            description,
        };
        Ok(Versioned::new(version, animal))
    }

    async fn create(&self, new: NewAnimal) -> RepositoryResult<Versioned<Animal>> {
        let id = AnimalId(Uuid::new_v4());
        let id_str = id.0.to_string();
        let animal = Animal {
            id,
            name: new.name,
            birthdate: new.birthdate,
            animal_species: new.animal_species,
            description: new.description,
        };
        let animal = Versioned::init(animal);
        let birthdate_str = new.birthdate.format("%Y-%m-%d").to_string();

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        tx.execute(
            "INSERT INTO animals (id, name, birthdate, animal_species, description, client_id, _version) VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6)",
            libsql::params![
                id_str,
                animal.name.clone(),
                birthdate_str,
                animal.animal_species.clone(),
                animal.description.clone(),
                animal.v(),
            ],
        )
        .await
        .contextualize_with(libsql_error_to_persistence_error)?;

        Ok(animal)
    }

    async fn update(&self, animal: &mut Versioned<Animal>) -> RepositoryResult<()> {
        let id_str = animal.id.0.to_string();
        let birthdate_str = animal.birthdate.format("%Y-%m-%d").to_string();
        let version = animal.v();

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "UPDATE animals SET name = ?1, birthdate = ?2, animal_species = ?3, description = ?4, _version = _version + 1 WHERE id = ?5 AND _version = ?6",
                libsql::params![
                    animal.name.clone(),
                    birthdate_str,
                    animal.animal_species.clone(),
                    animal.description.clone(),
                    id_str,
                    version,
                ],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if result != 1 {
            bail!(RepositoryError::VersionMismatch {
                expected: version,
                actual: None,
            });
        }

        animal.increment();
        Ok(())
    }

    async fn delete(&self, animal: Versioned<Animal>) -> RepositoryResult<()> {
        let id_str = animal.id.0.to_string();
        let version = animal.v();

        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut().ok_or(RepositoryError::Conflict)?;

        let result = tx
            .execute(
                "DELETE FROM animals WHERE id = ?1 AND _version = ?2",
                libsql::params![id_str, version],
            )
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;

        if result != 1 {
            bail!(RepositoryError::VersionMismatch {
                expected: version,
                actual: None,
            });
        }
        Ok(())
    }
}
