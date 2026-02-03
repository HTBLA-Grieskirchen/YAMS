use std::sync::Arc;

use async_trait::async_trait;
use libsql::Transaction;
use yams_core::{
    ports::repos::{
        AnimalRepository, ClientRepository, RepositoryResult, UnitOfWorkImpl, UnitOfWorkProvider,
    },
    service::errors::PersistenceError,
};

use crate::{
    SQLiteInstance,
    errors::ToPersistenceResultExt,
    repos::{SQLiteAnimalRepository, SQLiteClientRepository},
};

#[async_trait]
impl UnitOfWorkProvider for SQLiteInstance {
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkImpl>, PersistenceError> {
        todo!()
    }
}

pub struct SQLiteUnitOfWork {
    connection: Arc<libsql::Connection>,
    tx: Transaction,
}

#[async_trait]
impl UnitOfWorkImpl for SQLiteUnitOfWork {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        std::mem::replace(
            &mut self.tx,
            self.connection.transaction().await.to_persistence()?,
        )
        .commit()
        .await
        .to_persistence()?;
        Ok(())
    }

    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        self.tx.commit().await.to_persistence()
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        self.tx.rollback().await.to_persistence()
    }

    fn clients(&self) -> &dyn ClientRepository {
        &SQLiteClientRepository {

        }
    }

    fn animals(&self) -> &dyn AnimalRepository {
        &SQLiteAnimalRepository {}
    }
}
