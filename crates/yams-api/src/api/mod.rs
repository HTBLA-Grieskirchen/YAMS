pub mod requests;

use std::sync::Arc;

use yams_core::{
    App, ResultReport,
    application::ExecutionError,
    service::{CreateAnimal, CreateClient},
    uow::Versioned,
};

use crate::{
    requests::{AnimalCreation, ClientCreation},
    schema::{Animal, Client, schema_animal_from_domain, schema_client_from_domain},
};

#[derive(Clone)]
pub struct YamsAppApi {
    app: Arc<App>,
}

impl YamsAppApi {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }

    pub fn inner_app(&self) -> Arc<App> {
        self.app.clone()
    }
}

impl YamsAppApi {
    pub async fn create_client(
        &self,
        body: ClientCreation,
    ) -> ResultReport<Client, ExecutionError> {
        let client = self.app.execute(CreateClient::from(body)).await?;
        Ok(schema_client_from_domain(client, vec![]))
    }

    pub async fn create_animal(
        &self,
        body: AnimalCreation,
    ) -> ResultReport<Animal, ExecutionError> {
        let animal = self.app.execute(CreateAnimal::from(body)).await?;
        Ok(schema_animal_from_domain(animal))
    }

    pub async fn get_all_animals(&self) -> ResultReport<Vec<Animal>, ExecutionError> {
        let animals = self
            .app
            .execute_fn(async |ctx| ctx.uow.animals().find_all().await)
            .await?
            .into_iter()
            .map(Versioned::into_data);
        Ok(animals.map(schema_animal_from_domain).collect())
    }
}
