use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use std::cmp::Ordering;
use std::fmt::Debug;

use crate::application::{ResultReport, ports::RepositoryError};
use crate::ports::{AnimalRepository, ClientRepository, RepositoryResult};

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

    pub fn animals(&self) -> &dyn AnimalRepository {
        self.implementation.animals()
    }

    pub fn clients(&self) -> &dyn ClientRepository {
        self.implementation.clients()
    }
}

struct LockedUnitOfWorkImpl<'a> {
    inner: &'a dyn UnitOfWorkImpl,
}

#[async_trait]
impl UnitOfWorkImpl for LockedUnitOfWorkImpl<'_> {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        // Disabled for locked UoW, we cannot even due to borrow
        Ok(())
    }

    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        // Impossible because we only borrowed the outer UoW
        // No problem because only the outer one can be committed
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        // Impossible because we only borrowed the outer UoW
        // No problem because only the outer one can be rolled back
        Ok(())
    }

    fn clients(&self) -> &dyn ClientRepository {
        self.inner.clients()
    }

    fn animals(&self) -> &dyn AnimalRepository {
        self.inner.animals()
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

    fn clients(&self) -> &dyn ClientRepository {
        self.inner.clients()
    }

    fn animals(&self) -> &dyn AnimalRepository {
        self.inner.animals()
    }
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
