use std::fmt::Debug;

use async_trait::async_trait;
use chrono::NaiveDate;

use crate::application::{ResultReport, uow::Versioned};
use crate::domain::{
    Behandlung, BehandlungId, Haustier, HaustierId, Klient, KlientId, Leistung, LeistungId,
    LeistungOffen, Produkt, ProduktId, Rechnung, RechnungOffen, Seminar, SeminarId, SeminarTermin,
    SeminarTerminGeplant, SeminarTerminId, behandlung::NeueBehandlung, haustier::NeuesHaustier,
    klient::NeuerKlient, leistung::NeueLeistung, produkt::NeuesProdukt, seminar::NeuesSeminar,
    seminar_termin::NeuerSeminarTermin,
};

pub type RepositoryResult<T> = ResultReport<T, RepositoryError>;

#[async_trait]
pub trait KlientRepository: Send + Sync {
    async fn find_by_id(&self, id: KlientId) -> RepositoryResult<Versioned<Klient>>;
    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Klient>>>;
    async fn create(&self, klient: NeuerKlient) -> RepositoryResult<Versioned<Klient>>;
    async fn update(&self, klient: &mut Versioned<Klient>) -> RepositoryResult<()>;
    async fn delete(&self, klient: Versioned<Klient>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait HaustierRepository: Send + Sync {
    async fn find_by_id(&self, id: HaustierId) -> RepositoryResult<Versioned<Haustier>>;
    async fn find_by_klient_id(
        &self,
        klient_id: KlientId,
    ) -> RepositoryResult<Vec<Versioned<Haustier>>>;
    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Haustier>>>;
    async fn create(&self, haustier: NeuesHaustier) -> RepositoryResult<Versioned<Haustier>>;
    async fn update(&self, haustier: &mut Versioned<Haustier>) -> RepositoryResult<()>;
    async fn delete(&self, haustier: Versioned<Haustier>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait ProduktRepository: Send + Sync {
    async fn find_by_id(&self, id: ProduktId) -> RepositoryResult<Versioned<Produkt>>;
    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Produkt>>>;
    async fn create(&self, produkt: NeuesProdukt) -> RepositoryResult<Versioned<Produkt>>;
}

#[async_trait]
pub trait BehandlungRepository: Send + Sync {
    async fn find_by_id(&self, id: BehandlungId) -> RepositoryResult<Versioned<Behandlung>>;
    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Behandlung>>>;
    async fn create(&self, behandlung: NeueBehandlung) -> RepositoryResult<Versioned<Behandlung>>;
}

#[async_trait]
pub trait LeistungRepository: Send + Sync {
    async fn create(&self, leistung: NeueLeistung) -> RepositoryResult<Versioned<LeistungOffen>>;
    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Leistung>>>;
    async fn find_by_id(&self, id: LeistungId) -> RepositoryResult<Versioned<Leistung>>;
    async fn find_offene_by_datum(
        &self,
        datum: NaiveDate,
    ) -> RepositoryResult<Vec<Versioned<LeistungOffen>>>;
    async fn update(&self, leistung: &mut Versioned<Leistung>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait RechnungRepository: Send + Sync {
    async fn create(&self, rechnung: RechnungOffen) -> RepositoryResult<Versioned<RechnungOffen>>;
    async fn nächste_rechnungsnummer(&self) -> RepositoryResult<u64>;
    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Rechnung>>>;
    async fn find_by_klient_id(
        &self,
        klient_id: KlientId,
    ) -> RepositoryResult<Vec<Versioned<Rechnung>>>;
}

#[async_trait]
pub trait SeminarRepository: Send + Sync {
    async fn find_by_id(&self, id: SeminarId) -> RepositoryResult<Versioned<Seminar>>;
    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Seminar>>>;
    async fn create(&self, seminar: NeuesSeminar) -> RepositoryResult<Versioned<Seminar>>;
    async fn update(&self, seminar: &mut Versioned<Seminar>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait SeminarTerminRepository: Send + Sync {
    async fn find_by_id(&self, id: SeminarTerminId) -> RepositoryResult<Versioned<SeminarTermin>>;
    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<SeminarTermin>>>;
    async fn find_by_seminar_id(
        &self,
        seminar_id: SeminarId,
    ) -> RepositoryResult<Vec<Versioned<SeminarTermin>>>;
    async fn find_nicht_vollständig_abgerechnet_bis(
        &self,
        stichtag: NaiveDate,
    ) -> RepositoryResult<Vec<Versioned<SeminarTermin>>>;
    async fn create(
        &self,
        termin: NeuerSeminarTermin,
    ) -> RepositoryResult<Versioned<SeminarTerminGeplant>>;
    async fn update(&self, termin: &mut Versioned<SeminarTermin>) -> RepositoryResult<()>;
}

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,
    #[error("version mismatch - entity was modified by another process {expected} != {actual:?}")]
    VersionMismatch { expected: u64, actual: Option<u64> },
    #[error("conflict occurred")]
    Conflict,
    #[error("connection failed")]
    Connection,
    #[error("operation failed")]
    OperationFailed,
    #[error("permissions error")]
    Permission,
    #[error("storage error")]
    Storage,
    #[error("data error")]
    Data,
    #[error("unknown repository error")]
    Unknown,
}
