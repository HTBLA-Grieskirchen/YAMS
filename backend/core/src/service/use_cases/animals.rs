use async_trait::async_trait;
use chrono::NaiveDate;

use super::UseCase;
use crate::{
    application::OrchestratableError,
    domain::{Animal, ClientId, factories::NewAnimal},
    service::{ExecutionContext, errors::PersistenceError},
};

#[derive(Clone)]
pub struct CreateAnimal {
    pub client_id: ClientId,
    pub name: String,
    pub birthdate: NaiveDate,
    pub animal_species: String,
    pub description: String,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateAnimalError {
    #[error(transparent)]
    Persistence(#[from] PersistenceError),
}

impl OrchestratableError for CreateAnimalError {
    fn should_retry(&self) -> bool {
        match self {
            CreateAnimalError::Persistence(e) => e.should_retry(),
        }
    }
}

#[async_trait]
impl UseCase<Animal> for CreateAnimal {
    type Error = CreateAnimalError;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Animal, Self::Error> {
        let ExecutionContext { mut uow, .. } = ctx;

        let mut client = uow
            .clients()
            .find_by_id(self.client_id)
            .await?
            .ok_or(PersistenceError::NotFound)?;

        let animal = uow
            .animals()
            .create(NewAnimal {
                name: self.name,
                birthdate: self.birthdate,
                animal_species: self.animal_species,
                description: self.description,
            })
            .await?;

        uow.checkpoint().await?;

        client.animal_ids.push(animal.id.clone());
        uow.clients().update(&mut client).await?;

        Ok(animal.into_data())
    }
}

#[derive(Clone)]
pub struct CreateManyAnimals {
    pub animals: Vec<CreateAnimal>,
}

#[async_trait]
impl UseCase<Vec<Animal>> for CreateManyAnimals {
    type Error = CreateAnimalError;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Vec<Animal>, Self::Error> {
        let mut animals = Vec::with_capacity(self.animals.len());
        for fut in self.animals.into_iter().map(|a| a.perform(ctx.to_locked())) {
            animals.push(fut.await?)
        }
        Ok(animals)
    }
}
