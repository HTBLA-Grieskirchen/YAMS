use std::sync::Arc;

use async_lock::Mutex;
use async_trait::async_trait;
use libsql::Transaction;
use yams_core::{
    ErrorReportExt, ResultReport,
    ports::{
        BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository,
        ProduktRepository, RechnungRepository, RepositoryError, RepositoryResult,
        SeminarRepository, SeminarTerminRepository,
    },
    uow::{UnitOfWorkImpl, UnitOfWorkProvider},
};

use crate::{
    SQLiteConnection, SQLiteInstance,
    errors::libsql_error_to_persistence_error,
    repos::{
        SQLiteBehandlungRepository, SQLiteHaustierRepository, SQLiteKlientRepository,
        SQLiteLeistungRepository, SQLiteProduktRepository, SQLiteRechnungRepository,
        SQLiteSeminarRepository, SQLiteSeminarTerminRepository,
    },
};

pub struct SQLiteUnitOfWork {
    /// Held for the whole UoW so the connection outlives the transaction.
    /// Dropped after repos/tx (field order). Unused after `checkpoint` was removed.
    #[allow(dead_code)]
    connection: SQLiteConnection,
    pub(crate) tx: Arc<Mutex<Option<Transaction>>>,
    pub(crate) klient_repo: SQLiteKlientRepository,
    pub(crate) haustier_repo: SQLiteHaustierRepository,
    pub(crate) produkt_repo: SQLiteProduktRepository,
    pub(crate) behandlung_repo: SQLiteBehandlungRepository,
    pub(crate) leistung_repo: SQLiteLeistungRepository,
    pub(crate) rechnung_repo: SQLiteRechnungRepository,
    pub(crate) seminar_repo: SQLiteSeminarRepository,
    pub(crate) seminar_termin_repo: SQLiteSeminarTerminRepository,
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
            seminar_repo: SQLiteSeminarRepository { tx: tx.clone() },
            seminar_termin_repo: SQLiteSeminarTerminRepository { tx: tx.clone() },
            tx,
        }))
    }
}

#[async_trait]
impl UnitOfWorkImpl for SQLiteUnitOfWork {
    async fn commit(self: Box<Self>) -> RepositoryResult<()> {
        let mut tx_guard = self.tx.lock().await;
        let tx = tx_guard.take().ok_or(RepositoryError::Conflict)?;
        // No incremental_vacuum here: it needs an exclusive lock and races the
        // next connection open. auto_vacuum=INCREMENTAL still tracks free pages;
        // vacuum can run idle/shutdown if freelist reclaim is needed later.
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

    fn seminare(&self) -> &dyn SeminarRepository {
        &self.seminar_repo
    }

    fn seminar_termine(&self) -> &dyn SeminarTerminRepository {
        &self.seminar_termin_repo
    }
}
