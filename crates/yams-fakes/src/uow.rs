use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use yams_core::{
    ports::{
        BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository,
        ProduktRepository, RechnungRepository, RepositoryResult, SeminarRepository,
        SeminarTerminRepository,
    },
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

use crate::repository::{
    FakeBehandlungenRepository, FakeDatastore, FakeHaustiereRepository, FakeKlientenRepository,
    FakeLeistungenRepository, FakeProdukteRepository, FakeRechnungenRepository,
    FakeSeminarTermineRepository, FakeSeminareRepository,
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
    klienten: FakeKlientenRepository,
    haustiere: FakeHaustiereRepository,
    produkte: FakeProdukteRepository,
    behandlungen: FakeBehandlungenRepository,
    leistungen: FakeLeistungenRepository,
    rechnungen: FakeRechnungenRepository,
    seminare: FakeSeminareRepository,
    seminar_termine: FakeSeminarTermineRepository,
}

impl FakeUnitOfWork {
    pub fn new(log: Arc<Mutex<Vec<UoWEvent>>>, datastore: Arc<FakeDatastore>) -> Self {
        let backing_datastore = datastore;
        let snapshotted_datastore = FakeDatastore::clone(&backing_datastore);
        let transaction_datastore = Arc::new(snapshotted_datastore.clone());

        Self {
            log,
            klienten: FakeKlientenRepository::new(Arc::clone(&transaction_datastore)),
            haustiere: FakeHaustiereRepository::new(Arc::clone(&transaction_datastore)),
            produkte: FakeProdukteRepository::new(Arc::clone(&transaction_datastore)),
            behandlungen: FakeBehandlungenRepository::new(Arc::clone(&transaction_datastore)),
            leistungen: FakeLeistungenRepository::new(Arc::clone(&transaction_datastore)),
            rechnungen: FakeRechnungenRepository::new(Arc::clone(&transaction_datastore)),
            seminare: FakeSeminareRepository::new(Arc::clone(&transaction_datastore)),
            seminar_termine: FakeSeminarTermineRepository::new(Arc::clone(&transaction_datastore)),
            backing_datastore,
            snapshotted_datastore,
            transaction_datastore,
        }
    }
}

#[async_trait]
impl UnitOfWorkImpl for FakeUnitOfWork {
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

    fn klienten(&self) -> &dyn KlientRepository {
        &self.klienten
    }

    fn haustiere(&self) -> &dyn HaustierRepository {
        &self.haustiere
    }

    fn produkte(&self) -> &dyn ProduktRepository {
        &self.produkte
    }

    fn behandlungen(&self) -> &dyn BehandlungRepository {
        &self.behandlungen
    }

    fn leistungen(&self) -> &dyn LeistungRepository {
        &self.leistungen
    }

    fn rechnungen(&self) -> &dyn RechnungRepository {
        &self.rechnungen
    }

    fn seminare(&self) -> &dyn SeminarRepository {
        &self.seminare
    }

    fn seminar_termine(&self) -> &dyn SeminarTerminRepository {
        &self.seminar_termine
    }
}

#[derive(Debug)]
pub enum UoWEvent {
    Begin,
    Commit,
    Rollback,
    Error(String),
}
