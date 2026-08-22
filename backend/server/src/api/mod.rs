use std::sync::Arc;

use async_trait::async_trait;
use poem_openapi::payload::Json;
use yams_api::{
    YamsAppApi,
    errors::InternalServerError,
    requests::{AnimalCreation, ClientCreation},
    schema::{schema_animal_from_domain, schema_client_from_domain},
};
use yams_core::{
    App,
    service::{CreateAnimal, CreateClient},
};

pub struct AppApi {
    app: Arc<App>,
}

impl AppApi {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }
}
