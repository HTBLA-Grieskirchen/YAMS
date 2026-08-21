use std::sync::Arc;

use async_trait::async_trait;
use poem_openapi::payload::Json;
use yams_api::{YamsApi, errors::InternalServerError, requests::{AnimalCreation, ClientCreation}, responses::{CreateAnimalResponse, CreateClientResponse}, schema::{schema_animal_from_domain, schema_client_from_domain}};
use yams_core::{App, service::{CreateAnimal, CreateClient}};


pub struct AppApi {
    app: Arc<App>,
}

impl AppApi {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }
}

#[async_trait]
impl YamsApi for AppApi {
    async fn create_client(&self, body: ClientCreation) -> CreateClientResponse {
        let client = match self
            .app
            .execute(CreateClient::from(body))
            .await
        {
            Ok(client) => client,
            Err(e) => return CreateClientResponse::InternalError(Json(InternalServerError)),
        };
        CreateClientResponse::Ok(Json(schema_client_from_domain(client, vec![])))
    }

    async fn create_animal(&self, body: AnimalCreation) -> CreateAnimalResponse {
        let animal = match self
            .app
            .execute(CreateAnimal::from(body))
            .await
        {
            Ok(animal) => animal,
            Err(e) => return CreateAnimalResponse::InternalError(Json(InternalServerError)),
        };
        CreateAnimalResponse::Ok(Json(schema_animal_from_domain(animal)))
    }
}
