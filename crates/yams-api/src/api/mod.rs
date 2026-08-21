pub mod requests;
pub mod responses;
mod unimplemented;

use async_trait::async_trait;

pub use unimplemented::*;

use crate::{requests::{AnimalCreation, ClientCreation}, responses::{CreateAnimalResponse, CreateClientResponse}};

#[async_trait]
pub trait YamsApi: Send + Sync + 'static {
    async fn create_client(&self, body: ClientCreation) -> CreateClientResponse;

    async fn create_animal(&self, body: AnimalCreation) -> CreateAnimalResponse;
}
