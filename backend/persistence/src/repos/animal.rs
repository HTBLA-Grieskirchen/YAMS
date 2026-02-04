use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::{
    domain::{Animal, AnimalId, factories::NewAnimal},
    ports::repos::{AnimalRepository, RepositoryResult, Versioned},
};

use crate::errors::ToPersistenceResultExt;
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
    async fn find_by_id(&self, id: AnimalId) -> RepositoryResult<Option<Versioned<Animal>>> {
        let mut guard = self.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or(yams_core::service::errors::PersistenceError::ConcurrentModification)?;

        let id_str = id.0.to_string();
        let mut rows = tx
            .query(
                "SELECT id, name, birthdate, animal_species, description, _version FROM animals WHERE id = ?1",
                [id_str],
            )
            .await
            .to_persistence()?;

        let Some(row) = rows.next().await.to_persistence()? else {
            return Ok(None);
        };

        let id_raw: String = row.get(0).to_persistence()?;
        let name: String = row.get(1).to_persistence()?;
        let birthdate_str: String = row.get(2).to_persistence()?;
        let animal_species: String = row.get(3).to_persistence()?;
        let description: String = row.get(4).to_persistence()?;
        let version: u64 = row.get(5).to_persistence()?;

        let birthdate = parse_naive_date(&birthdate_str).map_err(|e| {
            yams_core::service::errors::PersistenceError::DeserializationError(anyhow::anyhow!(e))
        })?;
        let uuid = Uuid::from_str(&id_raw).map_err(|e| {
            yams_core::service::errors::PersistenceError::DeserializationError(anyhow::anyhow!(e))
        })?;

        let animal = Animal {
            id: AnimalId(uuid),
            name,
            birthdate,
            animal_species,
            description,
        };
        Ok(Some(Versioned::new(version, animal)))
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
        let tx = guard
            .as_mut()
            .ok_or(yams_core::service::errors::PersistenceError::ConcurrentModification)?;

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
        .to_persistence()?;

        Ok(animal)
    }

    async fn update(&self, animal: &mut Versioned<Animal>) -> RepositoryResult<()> {
        let id_str = animal.id.0.to_string();
        let birthdate_str = animal.birthdate.format("%Y-%m-%d").to_string();
        let version = animal.v();

        let mut guard = self.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or(yams_core::service::errors::PersistenceError::ConcurrentModification)?;

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
            .to_persistence()?;

        if result != 1 {
            return Err(
                yams_core::service::errors::PersistenceError::VersionMismatch {
                    expected: version,
                    actual: None,
                },
            );
        }

        animal.increment();
        Ok(())
    }

    async fn delete(&self, animal: Versioned<Animal>) -> RepositoryResult<()> {
        let id_str = animal.id.0.to_string();
        let version = animal.v();

        let mut guard = self.tx.lock().await;
        let tx = guard
            .as_mut()
            .ok_or(yams_core::service::errors::PersistenceError::ConcurrentModification)?;

        let result = tx
            .execute(
                "DELETE FROM animals WHERE id = ?1 AND _version = ?2",
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
