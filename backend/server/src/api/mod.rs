use std::sync::Arc;

use async_trait::async_trait;
use poem_openapi::payload::Json;
use yams_core::{
    App,
    service::{animals::CreateAnimal, client::CreateClient},
};
use yams_schema::{
    api::{
        AnimalCreation, Api, ApiImpl, ClientCreation, CreateAnimalResponse, CreateClientResponse,
    },
    errors::InternalServerError,
};

pub type AppApi = Api<AppApiImplementation>;

pub struct AppApiImplementation {
    app: Arc<App>,
}

impl AppApiImplementation {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }
}

#[async_trait]
impl ApiImpl for AppApiImplementation {
    async fn create_client(&self, body: ClientCreation) -> CreateClientResponse {
        let client = match self
            .app
            .execute(CreateClient::from(body))
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .map_err(InternalServerError::from)
        {
            Ok(client) => client,
            Err(e) => return CreateClientResponse::InternalError(Json(e)),
        };
        CreateClientResponse::Ok(Json(yams_schema::schema_client_from_domain(client, vec![])))
    }

    async fn create_animal(&self, body: AnimalCreation) -> CreateAnimalResponse {
        let animal = match self
            .app
            .execute(CreateAnimal::from(body))
            .await
            .map_err(|e| anyhow::anyhow!(e))
            .map_err(InternalServerError::from)
        {
            Ok(animal) => animal,
            Err(e) => return CreateAnimalResponse::InternalError(Json(e)),
        };
        CreateAnimalResponse::Ok(Json(yams_schema::schema_animal_from_domain(animal)))
    }
}
