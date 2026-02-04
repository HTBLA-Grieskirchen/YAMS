use std::sync::Arc;

use async_lock::{Mutex, MutexGuardArc};
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

pub struct SQLiteUnitOfWork {
    /// Held for the whole UoW; ensures only one transaction on the connection. Dropped on commit/rollback.
    connection: MutexGuardArc<libsql::Connection>,
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
    pub(crate) client_repo: SQLiteClientRepository,
    pub(crate) animal_repo: SQLiteAnimalRepository,
}

#[async_trait]
impl UnitOfWorkProvider for SQLiteInstance {
    async fn begin(&self) -> Result<Box<dyn UnitOfWorkImpl>, PersistenceError> {
        let connection = Arc::clone(&self.connection).lock_arc().await;
        let tx = connection
            .transaction_with_behavior(libsql::TransactionBehavior::Immediate)
            .await
            .to_persistence()?;
        let tx = Arc::new(Mutex::new(Some(tx)));
        let uow = SQLiteUnitOfWork {
            connection,
            client_repo: SQLiteClientRepository { tx: tx.clone() },
            animal_repo: SQLiteAnimalRepository { tx: tx.clone() },
            tx,
        };
        Ok(Box::new(uow))
    }
}

#[async_trait]
impl UnitOfWorkImpl for SQLiteUnitOfWork {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let old_tx = tx_guard
            .take()
            .ok_or(PersistenceError::ConcurrentModification)?;
        old_tx.commit().await.to_persistence()?;
        let new_tx = self
            .connection
            .transaction_with_behavior(libsql::TransactionBehavior::Immediate)
            .await
            .to_persistence()?;
        *tx_guard = Some(new_tx);
        Ok(())
    }

    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard
            .take()
            .ok_or(PersistenceError::ConcurrentModification)?;
        tx.commit().await.to_persistence()
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard
            .take()
            .ok_or(PersistenceError::ConcurrentModification)?;
        tx.rollback().await.to_persistence()
    }

    fn clients(&self) -> &dyn ClientRepository {
        &self.client_repo
    }

    fn animals(&self) -> &dyn AnimalRepository {
        &self.animal_repo
    }
}
