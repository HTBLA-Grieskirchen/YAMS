mod animal;
mod blank;
mod client;

pub use animal::*;
use async_trait::async_trait;
pub use client::*;
use poem_openapi::{OpenApi, payload::Json};

pub use blank::*;

pub struct Api<I: ApiImpl> {
    inner: I,
}

impl<I: ApiImpl> Api<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I: ApiImpl> From<I> for Api<I> {
    fn from(inner: I) -> Self {
        Self::new(inner)
    }
}

#[async_trait]
pub trait ApiImpl: Send + Sync + 'static {
    async fn create_client(&self, body: ClientCreation) -> CreateClientResponse;

    async fn create_animal(&self, body: AnimalCreation) -> CreateAnimalResponse;
}

#[OpenApi]
impl<I: ApiImpl> Api<I> {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    #[oai(path = "/client", method = "post")]
    async fn create_client(&self, body: Json<ClientCreation>) -> CreateClientResponse {
        self.inner.create_client(body.0).await
    }

    #[oai(path = "/animal", method = "post")]
    async fn create_animal(&self, body: Json<AnimalCreation>) -> CreateAnimalResponse {
        self.inner.create_animal(body.0).await
    }
}
