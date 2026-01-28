use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

use async_trait::async_trait;

use crate::domain::{
    Animal, AnimalId, Client, ClientId,
    factories::{NewAnimal, NewClient},
};

pub struct Versioned<T> {
    version: i64,
    data: T,
}

pub type RepositoryResult<T> = Result<T, crate::service::errors::PersistenceError>;

impl<T> Deref for Versioned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Versioned<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> PartialEq for Versioned<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl<T> PartialOrd for Versioned<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.version.partial_cmp(&other.version)
    }
}

impl<T> Versioned<T> {
    pub fn init(data: T) -> Self {
        Self { version: 0, data }
    }

    pub fn v(&self) -> i64 {
        self.version
    }

    pub fn incremented(self) -> Self {
        Self {
            version: self.version + 1,
            data: self.data,
        }
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn cloned_data(&self) -> T
    where
        T: Clone,
    {
        self.data.clone()
    }
}

impl<T> Clone for Versioned<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            version: self.version,
            data: self.cloned_data(),
        }
    }
}

impl<T> Debug for Versioned<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Versioned")
            .field("version", &self.version)
            .field("data", &self.data)
            .finish()
    }
}

#[async_trait]
pub trait ClientRepository: Send + Sync {
    async fn find_by_id(&self, id: ClientId) -> RepositoryResult<Option<Versioned<Client>>>;
    async fn create(&self, client: NewClient) -> RepositoryResult<Versioned<Client>>;
    async fn update(&self, client: Versioned<Client>) -> RepositoryResult<()>;
    async fn delete(&self, id: ClientId) -> RepositoryResult<()>;
}

#[async_trait]
pub trait AnimalRepository: Send + Sync {
    async fn find_by_id(&self, id: AnimalId) -> RepositoryResult<Option<Versioned<Animal>>>;
    async fn create(&self, animal: NewAnimal) -> RepositoryResult<Versioned<Animal>>;
    async fn update(&self, animal: Versioned<Animal>) -> RepositoryResult<()>;
    async fn delete(&self, id: AnimalId) -> RepositoryResult<()>;
}
