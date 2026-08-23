use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::Transaction;
use yams_core::{
    ErrorReportExt, ResultReport,
    ports::{
        BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository,
        ProduktRepository, RechnungRepository, RepositoryError, RepositoryResult,
    },
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

use crate::{
    SQLiteConnection, SQLiteInstance,
    errors::libsql_error_to_persistence_error,
    repos::{
        SQLiteBehandlungRepository, SQLiteHaustierRepository, SQLiteKlientRepository,
        SQLiteLeistungRepository, SQLiteProduktRepository, SQLiteRechnungRepository,
    },
};

pub struct SQLiteUnitOfWork {
    /// Held for the whole UoW; ensures only one transaction on the connection. Dropped on commit/rollback.
    connection: SQLiteConnection,
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
    pub(crate) klient_repo: SQLiteKlientRepository,
    pub(crate) haustier_repo: SQLiteHaustierRepository,
    pub(crate) produkt_repo: SQLiteProduktRepository,
    pub(crate) behandlung_repo: SQLiteBehandlungRepository,
    pub(crate) leistung_repo: SQLiteLeistungRepository,
    pub(crate) rechnung_repo: SQLiteRechnungRepository,
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
            klient_repo: SQLiteKlientRepository { tx: tx.clone() },
            haustier_repo: SQLiteHaustierRepository { tx: tx.clone() },
            produkt_repo: SQLiteProduktRepository { tx: tx.clone() },
            behandlung_repo: SQLiteBehandlungRepository { tx: tx.clone() },
            leistung_repo: SQLiteLeistungRepository { tx: tx.clone() },
            rechnung_repo: SQLiteRechnungRepository { tx: tx.clone() },
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

    fn klienten(&self) -> &dyn KlientRepository {
        &self.klient_repo
    }

    fn haustiere(&self) -> &dyn HaustierRepository {
        &self.haustier_repo
    }

    fn produkte(&self) -> &dyn ProduktRepository {
        &self.produkt_repo
    }

    fn behandlungen(&self) -> &dyn BehandlungRepository {
        &self.behandlung_repo
    }

    fn leistungen(&self) -> &dyn LeistungRepository {
        &self.leistung_repo
    }

    fn rechnungen(&self) -> &dyn RechnungRepository {
        &self.rechnung_repo
    }
}
