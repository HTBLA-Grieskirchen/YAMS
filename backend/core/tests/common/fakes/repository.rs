use std::{
    ops::DerefMut,
    sync::{Arc, Mutex, MutexGuard},
};

use async_trait::async_trait;
use fxhash::{FxHashMap, FxHashSet};
use uuid::Uuid;
use yams_core::{
    domain::factories::NewClient,
    ports::repos::{AnimalRepository, ClientRepository, RepositoryResult, Versioned},
    service::errors::PersistenceError,
};

use yams_core::domain::{Animal, AnimalId, Client, ClientId, factories::NewAnimal};

pub struct FakeDatastore {
    pub clients: Mutex<FxHashMap<Uuid, Versioned<Client>>>,
    pub animals: Mutex<FxHashMap<Uuid, Versioned<Animal>>>,
}

impl Clone for FakeDatastore {
    fn clone(&self) -> Self {
        Self {
            clients: Mutex::new(self.clients.lock().unwrap().clone()),
            animals: Mutex::new(self.animals.lock().unwrap().clone()),
        }
    }
}

impl FakeDatastore {
    pub fn new() -> Self {
        Self {
            clients: Mutex::new(FxHashMap::default()),
            animals: Mutex::new(FxHashMap::default()),
        }
    }

    pub fn replace_with(&self, other: &FakeDatastore) {
        *self.clients.lock().unwrap() = other.clients.lock().unwrap().clone();
        *self.animals.lock().unwrap() = other.animals.lock().unwrap().clone();
    }

    pub fn merge(
        target: &FakeDatastore,
        reference: &FakeDatastore,
        tx: &FakeDatastore,
    ) -> Result<FakeDatastore, PersistenceError> {
        fn acquire_map_lock<T>(
            map: &'_ Mutex<FxHashMap<Uuid, Versioned<T>>>,
        ) -> Result<MutexGuard<'_, FxHashMap<Uuid, Versioned<T>>>, PersistenceError> {
            map.lock().map_err(|_| PersistenceError::LockFailed)
        }

        let (mut target_clients, mut target_animals) = (
            acquire_map_lock(&target.clients)?,
            acquire_map_lock(&target.animals)?,
        );

        FakeDatastore::merge_single_aggregate(
            target_clients.deref_mut(),
            &*acquire_map_lock(&reference.clients)?,
            &*acquire_map_lock(&tx.clients)?,
        )?;
        FakeDatastore::merge_single_aggregate(
            target_animals.deref_mut(),
            &*acquire_map_lock(&reference.animals)?,
            &*acquire_map_lock(&tx.animals)?,
        )?;

        Ok(FakeDatastore {
            clients: Mutex::new(target_clients.clone()),
            animals: Mutex::new(target_animals.clone()),
        })
    }

    fn merge_single_aggregate<T: Clone>(
        target: &mut FxHashMap<Uuid, Versioned<T>>,
        reference: &FxHashMap<Uuid, Versioned<T>>,
        tx: &FxHashMap<Uuid, Versioned<T>>,
    ) -> Result<(), PersistenceError> {
        let all_ids = reference
            .keys()
            .chain(tx.keys())
            .cloned()
            .collect::<FxHashSet<_>>();

        for id in all_ids {
            let target_versioned = target.get(&id).cloned();
            let reference_versioned = reference.get(&id).cloned();
            let tx_versioned = tx.get(&id).cloned();

            match (reference_versioned, tx_versioned) {
                // Retained same entity in both tx and reference
                (Some(reference_versioned), Some(tx_versioned)) => {
                    // We need to update the target version
                    if reference_versioned.v() != tx_versioned.v() {
                        let Some(target_versioned) = target_versioned else {
                            return Err(PersistenceError::ConcurrentModification);
                        };
                        if target_versioned.v() > reference_versioned.v() {
                            return Err(PersistenceError::VersionMismatch {
                                expected: reference_versioned.v(),
                                actual: Some(target_versioned.v()),
                            });
                        }
                        target.insert(id, tx_versioned);
                    }
                }
                (Some(reference_versioned), None) => {
                    // We want to delete the entity from the target
                    if let Some(target_v) = target_versioned
                        && target_v.v() > reference_versioned.v()
                    {
                        return Err(PersistenceError::VersionMismatch {
                            expected: reference_versioned.v(),
                            actual: Some(target_v.v()),
                        });
                    }
                    target.remove(&id);
                }
                (None, Some(tx_versioned)) => {
                    // We want to insert the entity from the tx
                    if let Some(target_v) = target_versioned {
                        return Err(PersistenceError::ConcurrentModification);
                    }

                    target.insert(id, tx_versioned);
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub struct FakeAnimalsRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeAnimalsRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl AnimalRepository for FakeAnimalsRepository {
    async fn find_by_id(&self, id: AnimalId) -> RepositoryResult<Option<Versioned<Animal>>> {
        let data = self.datastore.animals.lock().unwrap();
        Ok(data.get(&id.0).cloned())
    }

    async fn create(&self, animal: NewAnimal) -> RepositoryResult<Versioned<Animal>> {
        let id = AnimalId(Uuid::new_v4());
        let mut data = self.datastore.animals.lock().unwrap();
        let versioned = Versioned::init(Animal {
            id,
            name: animal.name,
            birthdate: animal.birthdate,
            animal_species: animal.animal_species,
            description: animal.description,
        });
        data.insert(versioned.id.0.clone(), versioned.clone());
        Ok(versioned)
    }

    async fn update(&self, animal: &mut Versioned<Animal>) -> RepositoryResult<()> {
        let mut data = self.datastore.animals.lock().unwrap();
        if let Some(existing) = data.get(&animal.id.0) {
            if existing.v() != animal.v() {
                return Err(PersistenceError::VersionMismatch {
                    expected: existing.v(),
                    actual: Some(animal.v()),
                });
            }

            *animal = animal.clone().incremented();
            data.insert(animal.id.0.clone(), animal.clone());
            return Ok(());
        }
        Err(PersistenceError::NotFound)
    }

    async fn delete(&self, animal: Versioned<Animal>) -> RepositoryResult<()> {
        let mut data = self.datastore.animals.lock().unwrap();
        if let Some(existing) = data.get(&animal.id.0) {
            if existing.v() != animal.v() {
                return Err(PersistenceError::VersionMismatch {
                    expected: existing.v(),
                    actual: Some(animal.v()),
                });
            }
            data.remove(&animal.id.0);
            return Ok(());
        }
        Err(PersistenceError::NotFound)
    }
}

pub struct FakeClientsRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeClientsRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl ClientRepository for FakeClientsRepository {
    async fn find_by_id(&self, id: ClientId) -> RepositoryResult<Option<Versioned<Client>>> {
        let data = self.datastore.clients.lock().unwrap();
        Ok(data.get(&id.0).cloned())
    }

    async fn create(&self, client: NewClient) -> RepositoryResult<Versioned<Client>> {
        let id = ClientId(Uuid::new_v4());
        let mut data = self.datastore.clients.lock().unwrap();
        let versioned = Versioned::init(Client {
            id,
            first_name: client.first_name,
            last_name: client.last_name,
            birthdate: client.birthdate,
            email: client.email,
            mobile_number: client.mobile_number,
            customer_number: client.customer_number,
            consent: client.consent,
            address: client.address,
            animal_ids: Vec::new(),
        });
        data.insert(versioned.id.0.clone(), versioned.clone());

        Ok(versioned)
    }

    async fn update(&self, client: &mut Versioned<Client>) -> RepositoryResult<()> {
        let mut data = self.datastore.clients.lock().unwrap();
        if let Some(existing) = data.get(&client.id.0) {
            if existing.v() != client.v() {
                return Err(PersistenceError::VersionMismatch {
                    expected: existing.v(),
                    actual: Some(client.v()),
                });
            }
            *client = client.clone().incremented();
            data.insert(client.id.0.clone(), client.clone());
            return Ok(());
        }
        Err(PersistenceError::NotFound)
    }

    async fn delete(&self, client: Versioned<Client>) -> RepositoryResult<()> {
        let mut data = self.datastore.clients.lock().unwrap();
        if let Some(existing) = data.get(&client.id.0) {
            if existing.v() != client.v() {
                return Err(PersistenceError::VersionMismatch {
                    expected: existing.v(),
                    actual: Some(client.v()),
                });
            }
            data.remove(&client.id.0);
            return Ok(());
        }
        Err(PersistenceError::NotFound)
    }
}
