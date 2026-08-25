use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use std::cmp::Ordering;
use std::fmt::Debug;

use crate::application::{ResultReport, ports::RepositoryError};
use crate::ports::{
    BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository,
    ProduktRepository, RechnungRepository, RepositoryResult, SeminarRepository,
    SeminarTerminRepository,
};

pub struct UnitOfWork<'a> {
    implementation: Box<dyn UnitOfWorkImpl + 'a>,
}

impl<'a> UnitOfWork<'a> {
    pub fn new(implementation: Box<dyn UnitOfWorkImpl + 'a>) -> Self {
        Self { implementation }
    }
}

impl UnitOfWork<'_> {
    pub async fn checkpoint(&mut self) -> ResultReport<(), RepositoryError> {
        self.implementation.checkpoint().await
    }

    pub async fn commit(self) -> RepositoryResult<()> {
        self.implementation.commit().await
    }

    pub async fn rollback(self) -> RepositoryResult<()> {
        self.implementation.rollback().await
    }

    /// Create a new locked UoW, which is read-only and cannot be committed, checkpointed or rolled back.
    pub fn locked<'b>(&'b self) -> UnitOfWork<'b> {
        UnitOfWork {
            implementation: Box::new(LockedUnitOfWorkImpl {
                inner: self.implementation.as_ref(),
            }),
        }
    }

    /// Create a new shared UoW, which is can be checkpointed, but not consumed for commit or rollback.
    pub fn shared<'b>(&'b mut self) -> UnitOfWork<'b> {
        UnitOfWork {
            implementation: Box::new(SharedUnitOfWorkImpl {
                inner: self.implementation.as_mut(),
            }),
        }
    }

    pub fn klienten(&self) -> &dyn KlientRepository {
        self.implementation.klienten()
    }

    pub fn haustiere(&self) -> &dyn HaustierRepository {
        self.implementation.haustiere()
    }

    pub fn produkte(&self) -> &dyn ProduktRepository {
        self.implementation.produkte()
    }

    pub fn behandlungen(&self) -> &dyn BehandlungRepository {
        self.implementation.behandlungen()
    }

    pub fn leistungen(&self) -> &dyn LeistungRepository {
        self.implementation.leistungen()
    }

    pub fn rechnungen(&self) -> &dyn RechnungRepository {
        self.implementation.rechnungen()
    }

    pub fn seminare(&self) -> &dyn SeminarRepository {
        self.implementation.seminare()
    }

    pub fn seminar_termine(&self) -> &dyn SeminarTerminRepository {
        self.implementation.seminar_termine()
    }
}

struct LockedUnitOfWorkImpl<'a> {
    inner: &'a dyn UnitOfWorkImpl,
}

#[async_trait]
impl UnitOfWorkImpl for LockedUnitOfWorkImpl<'_> {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        Ok(())
    }

    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        Ok(())
    }

    fn klienten(&self) -> &dyn KlientRepository {
        self.inner.klienten()
    }

    fn haustiere(&self) -> &dyn HaustierRepository {
        self.inner.haustiere()
    }

    fn produkte(&self) -> &dyn ProduktRepository {
        self.inner.produkte()
    }

    fn behandlungen(&self) -> &dyn BehandlungRepository {
        self.inner.behandlungen()
    }

    fn leistungen(&self) -> &dyn LeistungRepository {
        self.inner.leistungen()
    }

    fn rechnungen(&self) -> &dyn RechnungRepository {
        self.inner.rechnungen()
    }

    fn seminare(&self) -> &dyn SeminarRepository {
        self.inner.seminare()
    }

    fn seminar_termine(&self) -> &dyn SeminarTerminRepository {
        self.inner.seminar_termine()
    }
}

struct SharedUnitOfWorkImpl<'a> {
    inner: &'a mut dyn UnitOfWorkImpl,
}

#[async_trait]
impl UnitOfWorkImpl for SharedUnitOfWorkImpl<'_> {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        self.inner.checkpoint().await
    }

    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        Ok(())
    }

    fn klienten(&self) -> &dyn KlientRepository {
        self.inner.klienten()
    }

    fn haustiere(&self) -> &dyn HaustierRepository {
        self.inner.haustiere()
    }

    fn produkte(&self) -> &dyn ProduktRepository {
        self.inner.produkte()
    }

    fn behandlungen(&self) -> &dyn BehandlungRepository {
        self.inner.behandlungen()
    }

    fn leistungen(&self) -> &dyn LeistungRepository {
        self.inner.leistungen()
    }

    fn rechnungen(&self) -> &dyn RechnungRepository {
        self.inner.rechnungen()
    }

    fn seminare(&self) -> &dyn SeminarRepository {
        self.inner.seminare()
    }

    fn seminar_termine(&self) -> &dyn SeminarTerminRepository {
        self.inner.seminar_termine()
    }
}

/// UoW Provider
#[async_trait]
pub trait UnitOfWorkImpl: Send + Sync {
    async fn checkpoint(&mut self) -> RepositoryResult<()>;
    async fn commit(self: Box<Self>) -> RepositoryResult<()>;
    async fn rollback(self: Box<Self>) -> RepositoryResult<()>;

    fn klienten(&self) -> &dyn KlientRepository;
    fn haustiere(&self) -> &dyn HaustierRepository;
    fn produkte(&self) -> &dyn ProduktRepository;
    fn behandlungen(&self) -> &dyn BehandlungRepository;
    fn leistungen(&self) -> &dyn LeistungRepository;
    fn rechnungen(&self) -> &dyn RechnungRepository;
    fn seminare(&self) -> &dyn SeminarRepository;
    fn seminar_termine(&self) -> &dyn SeminarTerminRepository;
}

#[async_trait]
pub trait UnitOfWorkProvider: Send + Sync {
    async fn begin(&self) -> RepositoryResult<Box<dyn UnitOfWorkImpl>>;
}

pub struct Versioned<T> {
    version: u64,
    data: T,
}

impl<T> Deref for Versioned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Versioned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> PartialEq for Versioned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl<T> PartialOrd for Versioned<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl<T> Versioned<T> {
    pub fn new(version: u64, data: T) -> Self {
        Self { version, data }
    }

    pub fn init(data: T) -> Self {
        Self { version: 0, data }
    }

    pub fn v(&self) -> u64 {
        self.version
    }

    pub fn increment(&mut self) -> u64 {
        self.version += 1;
        self.version
    }

    pub fn incremented(mut self) -> Self {
        self.increment();
        self
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn cloned_data(&self) -> T
    where
        T: Clone,
    {
        self.data.clone()
    }
}

impl<T> Clone for Versioned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            version: self.version,
            data: self.cloned_data(),
        }
    }
}

impl<T> Debug for Versioned<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Versioned")
            .field("version", &self.version)
            .field("data", &self.data)
            .finish()
    }
}
