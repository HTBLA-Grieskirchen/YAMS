use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::Transaction;
use yams_core::{
    ErrorReportExt, ResultReport,
    ports::{AnimalRepository, ClientRepository, RepositoryError, RepositoryResult},
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

use crate::{
    SQLiteConnection, SQLiteInstance,
    errors::libsql_error_to_persistence_error,
    repos::{SQLiteAnimalRepository, SQLiteClientRepository},
};

pub struct SQLiteUnitOfWork {
    /// Held for the whole UoW; ensures only one transaction on the connection. Dropped on commit/rollback.
    connection: SQLiteConnection,
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
    pub(crate) client_repo: SQLiteClientRepository,
    pub(crate) animal_repo: SQLiteAnimalRepository,
}

#[async_trait]
impl UnitOfWorkProvider for SQLiteInstance {
    async fn begin(&self) -> ResultReport<Box<dyn UnitOfWorkImpl>, RepositoryError> {
        let connection: SQLiteConnection = self.create_connection().await?;
        let tx = connection
            .transaction_with_behavior(libsql::TransactionBehavior::Deferred)
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        let tx = Arc::new(Mutex::new(Some(tx)));
        Ok(Box::new(SQLiteUnitOfWork {
            connection,
            client_repo: SQLiteClientRepository { tx: tx.clone() },
            animal_repo: SQLiteAnimalRepository { tx: tx.clone() },
            tx,
        }))
    }
}

#[async_trait]
impl UnitOfWorkImpl for SQLiteUnitOfWork {
    async fn checkpoint(&mut self) -> RepositoryResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let old_tx = tx_guard.take().ok_or(RepositoryError::Conflict)?;
        old_tx
            .commit()
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        let new_tx = self
            .connection
            .transaction_with_behavior(libsql::TransactionBehavior::Deferred)
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        *tx_guard = Some(new_tx);
        Ok(())
    }

    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.take().ok_or(RepositoryError::Conflict)?;
        tx.query("PRAGMA incremental_vacuum", ())
            .await
            .contextualize_with(libsql_error_to_persistence_error)?;
        tx.commit()
            .await
            .contextualize_with(libsql_error_to_persistence_error)
    }

    async fn rollback(self: Box<Self>) -> RepositoryResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.take().ok_or(RepositoryError::Conflict)?;
        tx.rollback()
            .await
            .contextualize_with(libsql_error_to_persistence_error)
    }

    fn clients(&self) -> &dyn ClientRepository {
        &self.client_repo
    }

    fn animals(&self) -> &dyn AnimalRepository {
        &self.animal_repo
    }
}
