use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use async_trait::async_trait;

use crate::domain::{
    Animal, AnimalId, Client, ClientId,
    factories::{NewAnimal, NewClient},
};
use crate::service::errors::PersistenceError;

pub type RepositoryResult<T> = Result<T, PersistenceError>;

pub trait RepositoryResultExt<T> {
    fn or_notfound(self) -> Result<T, PersistenceError>;
}

impl<T> RepositoryResultExt<T> for RepositoryResult<Option<T>> {
    fn or_notfound(self) -> Result<T, PersistenceError> {
        match self {
            Ok(Some(data)) => Ok(data),
            Ok(None) => Err(PersistenceError::NotFound),
            Err(e) => Err(e),
        }
    }
}

pub struct Versioned<T> {
    version: i64,
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
    pub fn new(version: i64, data: T) -> Self {
        Self { version, data }
    }

    pub fn init(data: T) -> Self {
        Self { version: 0, data }
    }

    pub fn v(&self) -> i64 {
        self.version
    }

    pub fn incremented(self) -> Self {
        Self {
            version: self.version + 1,
            data: self.data,
        }
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

#[async_trait]
pub trait ClientRepository: Send + Sync {
    async fn find_by_id(&self, id: ClientId) -> RepositoryResult<Option<Versioned<Client>>>;
    async fn create(&self, client: NewClient) -> RepositoryResult<Versioned<Client>>;
    async fn update(&self, client: &mut Versioned<Client>) -> RepositoryResult<()>;
    async fn delete(&self, client: Versioned<Client>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait AnimalRepository: Send + Sync {
    async fn find_by_id(&self, id: AnimalId) -> RepositoryResult<Option<Versioned<Animal>>>;
    async fn create(&self, animal: NewAnimal) -> RepositoryResult<Versioned<Animal>>;
    async fn update(&self, animal: &mut Versioned<Animal>) -> RepositoryResult<()>;
    async fn delete(&self, animal: Versioned<Animal>) -> RepositoryResult<()>;
}

/// UoW Provider
#[async_trait]
pub trait UnitOfWorkImpl: Send + Sync {
    /// Different from a commit, this allows further usage of this UoW afterwards. It guarantees that the changes are fully persisted, and can be read from anew upon full system restart.
    /// Beware, rollback after a checkpoint will not revert the changes, and the system will be in an inconsistent state. Rollback will only revert changes up to the latest checkpoint.
    /// Use sparingly, idealy for long running events such as batch pdf generation.
    async fn checkpoint(&mut self) -> RepositoryResult<()>;
    /// Commit the changes to the database, publishing them for all other UoW/transactions to see.
    /// This needs to ensure that upon success, the commit has already happened successfully.
    /// This needs to ensure that upon failure, the commit has not happened at all, following ACID
    /// rules.
    async fn commit(self: Box<Self>) -> RepositoryResult<()>;
    /// Rollback the changes to the database, undoing them for all other UoW/transactions to see.
    /// This needs to ensure that upon success, the rollback has already happened successfully.
    /// This needs to ensure that upon failure, the rollback has not happened at all, following ACID
    /// rules.
    async fn rollback(self: Box<Self>) -> RepositoryResult<()>;

    fn clients(&self) -> &dyn ClientRepository;
    fn animals(&self) -> &dyn AnimalRepository;
}

#[async_trait]
pub trait UnitOfWorkProvider: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkImpl>, PersistenceError>;
}
