use async_trait::async_trait;

use crate::{YamsApi, api::{AnimalCreation, ClientCreation, CreateAnimalResponse, CreateClientResponse}};

pub struct UnimplementedApi;

#[async_trait]
impl YamsApi for UnimplementedApi {
    async fn create_client(&self, _: ClientCreation) -> CreateClientResponse {
        unimplemented!()
    }

    async fn create_animal(&self, _: AnimalCreation) -> CreateAnimalResponse {
        unimplemented!()
    }
}
