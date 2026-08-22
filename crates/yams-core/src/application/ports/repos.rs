use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use error_stack::Report;

use crate::application::{ResultReport, uow::Versioned};
use crate::domain::{
    Animal, AnimalId, Client, ClientId,
    factories::{NewAnimal, NewClient},
};

pub type RepositoryResult<T> = ResultReport<T, RepositoryError>;

#[async_trait]
pub trait ClientRepository: Send + Sync {
    async fn find_by_id(&self, id: ClientId) -> RepositoryResult<Versioned<Client>>;
    async fn create(&self, client: NewClient) -> RepositoryResult<Versioned<Client>>;
    async fn update(&self, client: &mut Versioned<Client>) -> RepositoryResult<()>;
    async fn delete(&self, client: Versioned<Client>) -> RepositoryResult<()>;
}

#[async_trait]
pub trait AnimalRepository: Send + Sync {
    async fn find_by_id(&self, id: AnimalId) -> RepositoryResult<Versioned<Animal>>;
    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Animal>>>;
    async fn create(&self, animal: NewAnimal) -> RepositoryResult<Versioned<Animal>>;
    async fn update(&self, animal: &mut Versioned<Animal>) -> RepositoryResult<()>;
    async fn delete(&self, animal: Versioned<Animal>) -> RepositoryResult<()>;
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
