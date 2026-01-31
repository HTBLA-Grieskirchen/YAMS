use async_trait::async_trait;

use crate::{ports::repos::{AnimalRepository, ClientRepository, RepositoryResult, UnitOfWorkProvider}, service::{errors::PersistenceError, ports::repos::UnitOfWorkImpl}};


pub struct UnitOfWork {
    implementation: Box<dyn UnitOfWorkImpl>,
}

impl UnitOfWork {
    pub fn new(implementation: Box<dyn UnitOfWorkImpl>) -> Self {
        Self { implementation }
    }
}

impl UnitOfWork {
    pub async fn checkpoint(&mut self) -> Result<(), PersistenceError> {
        self.implementation.checkpoint().await
    }

    pub async fn commit(self) -> Result<(), PersistenceError> {
        self.implementation.commit().await
    }
    
    pub async fn rollback(self) -> Result<(), PersistenceError> {
        self.implementation.rollback().await
    }

    pub fn locked(&self) -> &UnitOfWork {
        &UnitOfWork::new(Box::new(LockedUnitOfWorkImpl { uow: self }))
    }

    pub fn animals(&self) -> &dyn AnimalRepository {
        self.implementation.animals()
    }

    pub fn clients(&self) -> &dyn ClientRepository {
        self.implementation.clients()
    }
}

pub struct LockedUnitOfWorkImpl<'a> {
    uow: &'a UnitOfWork,
}

#[async_trait]
impl UnitOfWorkImpl for LockedUnitOfWorkImpl<'_> {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        // Disabled for locked UoW
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
    
    fn clients(&self) ->  &dyn ClientRepository {
        self.uow.clients()
    }
    
    fn animals(&self) ->  &dyn AnimalRepository {
        self.uow.animals()
    }
}

