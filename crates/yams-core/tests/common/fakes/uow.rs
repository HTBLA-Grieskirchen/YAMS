use std::sync::{Arc, Mutex};

use crate::common::fakes::repository::{
    FakeAnimalsRepository, FakeClientsRepository, FakeDatastore,
};
use async_trait::async_trait;
use yams_core::{
    ports::{AnimalRepository, ClientRepository, RepositoryResult},
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

pub struct FakeUnitOfWorkProvider {
    pub log: Arc<Mutex<Vec<UoWEvent>>>,
    pub datastore: Arc<FakeDatastore>,
}

impl FakeUnitOfWorkProvider {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self {
            log: Arc::new(Mutex::new(Vec::new())),
            datastore,
        }
    }

    pub fn empty() -> Self {
        Self::new(Arc::new(FakeDatastore::new()))
    }
}

#[async_trait]
impl UnitOfWorkProvider for FakeUnitOfWorkProvider {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>> {
        self.log.lock().unwrap().push(UoWEvent::Begin);
        Ok(Box::new(FakeUnitOfWork::new(
            Arc::clone(&self.log),
            Arc::clone(&self.datastore),
        )))
    }
}

pub struct FakeUnitOfWork {
    log: Arc<Mutex<Vec<UoWEvent>>>,
    backing_datastore: Arc<FakeDatastore>,
    snapshotted_datastore: FakeDatastore,
    transaction_datastore: Arc<FakeDatastore>,
    animals: FakeAnimalsRepository,
    clients: FakeClientsRepository,
}

impl FakeUnitOfWork {
    pub fn new(log: Arc<Mutex<Vec<UoWEvent>>>, datastore: Arc<FakeDatastore>) -> Self {
        let backing_datastore = datastore;
        let snapshotted_datastore = FakeDatastore::clone(&backing_datastore);
        let transaction_datastore = Arc::new(snapshotted_datastore.clone());

        Self {
            log,
            animals: FakeAnimalsRepository::new(Arc::clone(&transaction_datastore)),
            clients: FakeClientsRepository::new(Arc::clone(&transaction_datastore)),
            backing_datastore,
            snapshotted_datastore,
            transaction_datastore,
        }
    }
}

#[async_trait]
impl UnitOfWorkImpl for FakeUnitOfWork {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        self.log.lock().unwrap().push(UoWEvent::Checkpoint);

        let new_snapshot = FakeDatastore::merge(
            &self.backing_datastore,
            &self.snapshotted_datastore,
            &self.transaction_datastore,
        )
        .inspect_err(|e| {
            self.log
                .lock()
                .unwrap()
                .push(UoWEvent::Error(e.to_string()))
        })?;

        self.snapshotted_datastore = new_snapshot;

        Ok(())
    }

    async fn commit(mut self: Box<Self>) -> RepositoryResult<()> {
        self.log.lock().unwrap().push(UoWEvent::Commit);

        let new_snapshot = FakeDatastore::merge(
            &self.backing_datastore,
            &self.snapshotted_datastore,
            &self.transaction_datastore,
        )
        .inspect_err(|e| {
            self.log
                .lock()
                .unwrap()
                .push(UoWEvent::Error(e.to_string()))
        })?;

        self.transaction_datastore.replace_with(&new_snapshot);
        self.snapshotted_datastore = new_snapshot;

        Ok(())
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        self.log.lock().unwrap().push(UoWEvent::Rollback);
        Ok(())
    }

    fn animals(&self) -> &dyn AnimalRepository {
        &self.animals
    }

    fn clients(&self) -> &dyn ClientRepository {
        &self.clients
    }
}

#[derive(Debug)]
pub enum UoWEvent {
    Begin,
    Checkpoint,
    Commit,
    Rollback,
    Error(String),
}
