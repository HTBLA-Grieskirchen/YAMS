use crate::YamsApi;

pub struct UnimplementedApi;

impl YamsApi for UnimplementedApi {
    async fn create_client(&self, _: ClientCreation) -> CreateClientResponse {
        unimplemented!()
    }

    async fn create_animal(&self, _: AnimalCreation) -> CreateAnimalResponse {
        unimplemented!()
    }
}
