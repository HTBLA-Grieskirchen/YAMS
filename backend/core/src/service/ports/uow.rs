use crate::{
    ports::repos::{AnimalRepository, ClientRepository},
    service::errors::PersistenceError,
};
use async_trait::async_trait;

#[async_trait]
pub trait UnitOfWorkProvider: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, PersistenceError>;
}

#[derive(Clone)]
pub struct LockedUnitOfWork<'a, UoW: UnitOfWork> {
    uow: &'a UoW,
}

#[async_trait]
impl<UoW: UnitOfWork> UnitOfWork for LockedUnitOfWork<'_, UoW> {
    async fn commit(&mut self) -> Result<(), PersistenceError> {
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), PersistenceError> {
        Ok(())
    }

    fn animals(&self) -> &dyn AnimalRepository {
        self.uow.animals()
    }

    fn clients(&self) -> &dyn ClientRepository {
        self.uow.clients()
    }
}

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    /// Commits the unit work, must be implemented in a way that allows consecutive calls.
    async fn commit(&mut self) -> Result<(), PersistenceError>;
    /// Rolls back the unit work, must be implemented in a way that allows invocation after a commit.
    async fn rollback(self: Box<Self>) -> Result<(), PersistenceError>;

    fn animals(&self) -> &dyn AnimalRepository;
    fn clients(&self) -> &dyn ClientRepository;
}

#[async_trait]
pub trait UnitOfWorkExt: UnitOfWork + Sized {
    /// Returns a locked unit of work, that does nothing on commit or rollback. Can be used to
    /// invoke other use cases, without giving them ownership over transaction management.
    async fn locked(&self) -> LockedUnitOfWork<'_, Self>;
}

#[async_trait]
impl<T: UnitOfWork> UnitOfWorkExt for T {
    async fn locked(&self) -> LockedUnitOfWork<'_, Self> {
        LockedUnitOfWork { uow: self }
    }
}
