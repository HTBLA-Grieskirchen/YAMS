use async_trait::async_trait;
use libsql::Transaction;
use yams_core::{ports::repos::{AnimalRepository, ClientRepository, RepositoryResult, UnitOfWorkImpl, UnitOfWorkProvider}, service::errors::PersistenceError};

use crate::{SQLiteInstance, repos::{SQLiteAnimalRepository, SQLiteClientRepository}};

#[async_trait]
impl UnitOfWorkProvider for SQLiteInstance {
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkImpl>, PersistenceError> {
        todo!()
    }
}

pub struct SQLiteUnitOfWork {
    pub(crate) tx: Transaction,
}

#[async_trait]
impl UnitOfWorkImpl for SQLiteUnitOfWork {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        todo!()
    }
    
    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        todo!()
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        todo!()
    }
    
    fn clients(&self) -> &dyn ClientRepository {
        &SQLiteClientRepository {

        }
    }

    fn animals(&self) -> &dyn AnimalRepository {
        &SQLiteAnimalRepository {

        }
    }
}
