use async_trait::async_trait;

use crate::{
    ports::repos::{AnimalRepository, ClientRepository, RepositoryResult},
    service::{errors::PersistenceError, ports::repos::UnitOfWorkImpl},
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
    pub async fn checkpoint(&mut self) -> Result<(), PersistenceError> {
        self.implementation.checkpoint().await
    }

    pub async fn commit(self) -> Result<(), PersistenceError> {
        self.implementation.commit().await
    }

    pub async fn rollback(self) -> Result<(), PersistenceError> {
        self.implementation.rollback().await
    }

    pub fn locked<'b>(&'b self) -> UnitOfWork<'b> {
        UnitOfWork {
            implementation: Box::new(LockedUnitOfWorkImpl {
                inner: self.implementation.as_ref(),
            }),
        }
    }

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
