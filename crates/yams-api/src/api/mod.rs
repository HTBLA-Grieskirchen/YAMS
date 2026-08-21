pub mod requests;
pub mod responses;
mod unimplemented;

pub use animal::*;
use async_trait::async_trait;
pub use client::*;

pub use unimplemented::*;

pub struct Api<I: YamsApi> {
    inner: I,
}

impl<I: YamsApi> Api<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I: YamsApi> From<I> for Api<I> {
    fn from(inner: I) -> Self {
        Self::new(inner)
    }
}

#[async_trait]
pub trait YamsApi: Send + Sync + 'static {
    async fn create_client(&self, body: ClientCreation) -> CreateClientResponse;

    async fn create_animal(&self, body: AnimalCreation) -> CreateAnimalResponse;
}
