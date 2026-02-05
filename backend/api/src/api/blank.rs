use async_trait::async_trait;

use crate::api::{
    AnimalCreation, ApiImpl, ClientCreation, CreateAnimalResponse, CreateClientResponse,
};

pub struct UnimplementedApi;

#[async_trait]
impl ApiImpl for UnimplementedApi {
    async fn create_client(&self, _: ClientCreation) -> CreateClientResponse {
        unimplemented!()
    }

    async fn create_animal(&self, _: AnimalCreation) -> CreateAnimalResponse {
        unimplemented!()
    }
}
