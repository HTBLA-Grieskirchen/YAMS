use async_trait::async_trait;
use libsql::{params, Transaction};
use yams_core::domain::{Animal, AnimalId, Client, ClientId, Email, MobileNumber, Address};
use yams_core::domain::factories::{NewAnimal, NewClient};
use yams_core::service::errors::PersistenceError;
use yams_core::service::ports::repos::{
    AnimalRepository, ClientRepository, RepositoryResult, UnitOfWorkImpl, UnitOfWorkProvider,
    Versioned,
};
use uuid::Uuid;
use chrono::NaiveDate;

pub struct LibSqlUnitOfWork {
    pub(crate) tx: Transaction,
}

#[async_trait]
impl UnitOfWorkImpl for LibSqlUnitOfWork {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        self.tx.execute("SAVEPOINT checkpoint", ())
            .await
            .map_err(|e| PersistenceError::TransactionFailed(anyhow::anyhow!(e)))?;
        Ok(())
    }

    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        self.tx.commit()
            .await
            .map_err(|e| PersistenceError::TransactionFailed(anyhow::anyhow!(e)))?;
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        self.tx.rollback()
            .await
            .map_err(|e| PersistenceError::TransactionFailed(anyhow::anyhow!(e)))?;
        Ok(())
    }

    fn clients(&self) -> &dyn ClientRepository {
        self
    }

    fn animals(&self) -> &dyn AnimalRepository {
        self
    }
}

#[async_trait]
impl ClientRepository for LibSqlUnitOfWork {
    async fn find_by_id(&self, id: ClientId) -> RepositoryResult<Option<Versioned<Client>>> {
        let mut rows = self.tx.query(
            "SELECT c.id, c.first_name, c.last_name, c.birthdate, c.email, c.mobile_number, c.customer_number, c.consent, c.version,
                    a.postal_code, a.city, a.street, a.street_number, a.country
             FROM clients c
             JOIN addresses a ON c.address_id = a.id
             WHERE c.id = ?1",
            params![id.0.to_string()],
        )
        .await
        .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

        if let Some(row) = rows.next().await.map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))? {
            let id_str: String = row.get(0).unwrap();
            let first_name: String = row.get(1).unwrap();
            let last_name: String = row.get(2).unwrap();
            let birthdate_str: String = row.get(3).unwrap();
            let email: String = row.get(4).unwrap();
            let mobile_number: String = row.get(5).unwrap();
            let customer_number: i64 = row.get(6).unwrap();
            let consent: bool = row.get(7).unwrap();
            let version: i64 = row.get(8).unwrap();
            
            let postal_code: String = row.get(9).unwrap();
            let city: String = row.get(10).unwrap();
            let street: String = row.get(11).unwrap();
            let street_number: String = row.get(12).unwrap();
            let country_code: String = row.get(13).unwrap();

            let birthdate = NaiveDate::parse_from_str(&birthdate_str, "%Y-%m-%d")
                .map_err(|e| PersistenceError::DeserializationError(anyhow::anyhow!(e)))?;

            // Get animal_ids
            let mut animal_rows = self.tx.query(
                "SELECT animal_id FROM client_animals WHERE client_id = ?1",
                params![id_str.clone()],
            )
            .await
            .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

            let mut animal_ids = Vec::new();
            while let Some(a_row) = animal_rows.next().await.map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))? {
                let a_id_str: String = a_row.get(0).unwrap();
                animal_ids.push(AnimalId(Uuid::parse_str(&a_id_str).unwrap()));
            }

            let client = Client {
                id: ClientId(Uuid::parse_str(&id_str).unwrap()),
                first_name,
                last_name,
                birthdate,
                email: Email(email),
                mobile_number: MobileNumber(mobile_number),
                customer_number,
                consent,
                address: Address {
                    postal_code,
                    city,
                    street_and_number: format!("{} {}", street, street_number).trim().to_string(),
                    country_code,
                },
                animal_ids,
            };

            Ok(Some(Versioned::new(version, client)))
        } else {
            Ok(None)
        }
    }

    async fn create(&self, client: NewClient) -> RepositoryResult<Versioned<Client>> {
        let id = Uuid::new_v4();
        let address_id = Uuid::new_v4();

        self.tx.execute(
            "INSERT INTO addresses (id, postal_code, city, street, street_number, extra, country)
             VALUES (?1, ?2, ?3, ?4, '', '', ?5)",
            params![
                address_id.to_string(), 
                client.address.postal_code.clone(), 
                client.address.city.clone(), 
                client.address.street_and_number.clone(), 
                client.address.country_code.clone()
            ],
        )
        .await
        .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

        self.tx.execute(
            "INSERT INTO clients (id, first_name, last_name, birthdate, email, mobile_number, customer_number, address_id, consent, version)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 0)",
            params![
                id.to_string(),
                client.first_name.clone(),
                client.last_name.clone(),
                client.birthdate.format("%Y-%m-%d").to_string(),
                client.email.0.clone(),
                client.mobile_number.0.clone(),
                client.customer_number,
                address_id.to_string(),
                client.consent,
            ],
        )
        .await
        .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

        let client_data = Client {
            id: ClientId(id),
            first_name: client.first_name,
            last_name: client.last_name,
            birthdate: client.birthdate,
            email: client.email,
            mobile_number: client.mobile_number,
            customer_number: client.customer_number,
            consent: client.consent,
            address: client.address,
            animal_ids: Vec::new(),
        };

        Ok(Versioned::init(client_data))
    }

    async fn update(&self, client: &mut Versioned<Client>) -> RepositoryResult<()> {
        let old_version = client.v();
        let new_version = old_version + 1;

        let res = self.tx.execute(
            "UPDATE clients SET 
                first_name = ?1, last_name = ?2, birthdate = ?3, email = ?4, mobile_number = ?5, 
                customer_number = ?6, consent = ?7, version = ?8
             WHERE id = ?9 AND version = ?10",
            params![
                client.first_name.clone(),
                client.last_name.clone(),
                client.birthdate.format("%Y-%m-%d").to_string(),
                client.email.0.clone(),
                client.mobile_number.0.clone(),
                client.customer_number,
                client.consent,
                new_version,
                client.id.0.to_string(),
                old_version,
            ],
        )
        .await
        .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

        if res == 0 {
            return Err(PersistenceError::VersionMismatch { expected: old_version, actual: -1 });
        }

        // Update address as well
        let mut rows = self.tx.query("SELECT address_id FROM clients WHERE id = ?1", params![client.id.0.to_string()])
            .await
            .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;
        
        if let Some(row) = rows.next().await.map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))? {
            let address_id: String = row.get(0).unwrap();
            self.tx.execute(
                "UPDATE addresses SET postal_code = ?1, city = ?2, street = ?3, country = ?4 WHERE id = ?5",
                params![
                    client.address.postal_code.clone(), 
                    client.address.city.clone(), 
                    client.address.street_and_number.clone(), 
                    client.address.country_code.clone(), 
                    address_id
                ],
            )
            .await
            .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;
        }

        // Update client_animals
        self.tx.execute("DELETE FROM client_animals WHERE client_id = ?1", params![client.id.0.to_string()])
            .await
            .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;
        
        for animal_id in &client.animal_ids {
            self.tx.execute(
                "INSERT INTO client_animals (client_id, animal_id) VALUES (?1, ?2)",
                params![client.id.0.to_string(), animal_id.0.to_string()],
            )
            .await
            .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;
        }

        *client = client.clone().incremented();
        Ok(())
    }

    async fn delete(&self, client: Versioned<Client>) -> RepositoryResult<()> {
        let mut rows = self.tx.query("SELECT address_id FROM clients WHERE id = ?1", params![client.id.0.to_string()])
            .await
            .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;
        
        let address_id: Option<String> = rows.next().await.map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?.map(|row| row.get(0).unwrap());

        let res = self.tx.execute(
            "DELETE FROM clients WHERE id = ?1 AND version = ?2",
            params![client.id.0.to_string(), client.v()],
        )
        .await
        .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

        if res == 0 {
            return Err(PersistenceError::VersionMismatch { expected: client.v(), actual: -1 });
        }

        if let Some(aid) = address_id {
            self.tx.execute("DELETE FROM addresses WHERE id = ?1", params![aid])
                .await
                .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;
        }

        Ok(())
    }
}

#[async_trait]
impl AnimalRepository for LibSqlUnitOfWork {
    async fn find_by_id(&self, id: AnimalId) -> RepositoryResult<Option<Versioned<Animal>>> {
        let mut rows = self.tx.query(
            "SELECT id, name, birthdate, animal_species, description, version FROM animals WHERE id = ?1",
            params![id.0.to_string()],
        )
        .await
        .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

        if let Some(row) = rows.next().await.map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))? {
            let id_str: String = row.get(0).unwrap();
            let name: String = row.get(1).unwrap();
            let birthdate_str: String = row.get(2).unwrap();
            let animal_species: String = row.get(3).unwrap();
            let description: String = row.get(4).unwrap();
            let version: i64 = row.get(5).unwrap();

            let birthdate = NaiveDate::parse_from_str(&birthdate_str, "%Y-%m-%d")
                .map_err(|e| PersistenceError::DeserializationError(anyhow::anyhow!(e)))?;

            let animal = Animal {
                id: AnimalId(Uuid::parse_str(&id_str).unwrap()),
                name,
                birthdate,
                animal_species,
                description,
            };

            Ok(Some(Versioned::new(version, animal)))
        } else {
            Ok(None)
        }
    }

    async fn create(&self, animal: NewAnimal) -> RepositoryResult<Versioned<Animal>> {
        let id = Uuid::new_v4();

        self.tx.execute(
            "INSERT INTO animals (id, name, birthdate, animal_species, description, version)
             VALUES (?1, ?2, ?3, ?4, ?5, 0)",
            params![
                id.to_string(),
                animal.name.clone(),
                animal.birthdate.format("%Y-%m-%d").to_string(),
                animal.animal_species.clone(),
                animal.description.clone(),
            ],
        )
        .await
        .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

        let animal_data = Animal {
            id: AnimalId(id),
            name: animal.name,
            birthdate: animal.birthdate,
            animal_species: animal.animal_species,
            description: animal.description,
        };

        Ok(Versioned::init(animal_data))
    }

    async fn update(&self, animal: &mut Versioned<Animal>) -> RepositoryResult<()> {
        let old_version = animal.v();
        let new_version = old_version + 1;

        let res = self.tx.execute(
            "UPDATE animals SET name = ?1, birthdate = ?2, animal_species = ?3, description = ?4, version = ?5 WHERE id = ?6 AND version = ?7",
            params![
                animal.name.clone(),
                animal.birthdate.format("%Y-%m-%d").to_string(),
                animal.animal_species.clone(),
                animal.description.clone(),
                new_version,
                animal.id.0.to_string(),
                old_version,
            ],
        )
        .await
        .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

        if res == 0 {
            return Err(PersistenceError::VersionMismatch { expected: old_version, actual: -1 });
        }

        *animal = animal.clone().incremented();
        Ok(())
    }

    async fn delete(&self, animal: Versioned<Animal>) -> RepositoryResult<()> {
        let res = self.tx.execute(
            "DELETE FROM animals WHERE id = ?1 AND version = ?2",
            params![animal.id.0.to_string(), animal.v()],
        )
        .await
        .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;

        if res == 0 {
            return Err(PersistenceError::VersionMismatch { expected: animal.v(), actual: -1 });
        }

        Ok(())
    }
}

pub struct LibSqlUnitOfWorkProvider {
    pub(crate) conn: libsql::Connection,
}

impl LibSqlUnitOfWorkProvider {
    pub fn new(conn: libsql::Connection) -> Self {
        Self { conn }
    }

    pub fn conn(&self) -> &libsql::Connection {
        &self.conn
    }
}

#[async_trait]
impl UnitOfWorkProvider for LibSqlUnitOfWorkProvider {
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkImpl>, PersistenceError> {
        let tx = self.conn.transaction().await
            .map_err(|e| PersistenceError::Unknown(anyhow::anyhow!(e)))?;
        Ok(Box::new(LibSqlUnitOfWork { tx }))
    }
}
