use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{IntoReport, Report, ResultExt, bail};

use crate::{
    ResultReport, domain::{Animal, ClientId, factories::NewAnimal}, service::{ExecutionContext, UseCase},
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
    #[error("persistence error occurred")]
    Persistence,
    #[error("client with id `{}` not found", 0.0)]
    ClientNotFound(ClientId),
}

#[async_trait]
impl UseCase<Animal> for CreateAnimal {
    type Error = CreateAnimalError;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Animal, Report<Self::Error>> {
        let ExecutionContext { mut uow, .. } = ctx;

        let mut client = uow
            .clients()
            .find_by_id(self.client_id.clone())
            .await
            .change_context(CreateAnimalError::ClientNotFound(self.client_id.clone()))?;

        let animal = uow
            .animals()
            .create(NewAnimal {
                name: self.name,
                birthdate: self.birthdate,
                animal_species: self.animal_species,
                description: self.description,
            })
            .await
            .change_context(CreateAnimalError::Persistence)?;

        uow.checkpoint()
            .await
            .change_context(CreateAnimalError::Persistence)?;

        client.animal_ids.push(animal.id.clone());
        uow.clients()
            .update(&mut client)
            .await
            .change_context(CreateAnimalError::Persistence)?;

        Ok(animal.into_data())
    }
}

#[derive(Clone)]
pub struct CreateManyAnimals {
    pub animals: Vec<CreateAnimal>,
}

#[derive(thiserror::Error, Debug)]
#[error("failed to create {failures} out of {target} animals")]
pub struct CreateManyAnimalsError {
    failures: usize,
    target: usize,
}

#[async_trait]
impl UseCase<Vec<Animal>> for CreateManyAnimals {
    type Error = Report<[CreateAnimalError]>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> ResultReport<Vec<Animal>, <Self::Error as IntoReport>::Context> {
        let mut errors = Option::<Report<[CreateAnimalError]>>::None;
        let mut animals = Vec::with_capacity(self.animals.len());
        for fut in self.animals.into_iter().map(|a| a.perform(ctx.to_locked())) {
            match fut.await {
                Ok(animal) => animals.push(animal),
                Err(e) => match &mut errors {
                    Some(errors) => errors.push(e),
                    None => errors = Some(e.expand()),
                },
            }
        }
        if let Some(errors) = errors {
            return Err(errors);
        }
        Ok(animals)
    }
}
