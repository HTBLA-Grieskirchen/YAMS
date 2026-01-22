use std::sync::Arc;
use uuid::Uuid;
use crate::models::{Animal, NewAnimal, Race, NewRace};
use crate::ports::{AnimalRepository, RaceRepository};
use crate::error::Result;

pub struct AnimalService {
    repo: Arc<dyn AnimalRepository>,
}

impl AnimalService {
    pub fn new(repo: Arc<dyn AnimalRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Animal>> {
        self.repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Animal>> {
        self.repo.find_by_id(id).await
    }

    pub async fn create(&self, animal: NewAnimal) -> Result<Animal> {
        self.repo.create(animal).await
    }

    pub async fn update(&self, animal: Animal) -> Result<Animal> {
        self.repo.update(animal).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await
    }
}

pub struct RaceService {
    repo: Arc<dyn RaceRepository>,
}

impl RaceService {
    pub fn new(repo: Arc<dyn RaceRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Race>> {
        self.repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Race>> {
        self.repo.find_by_id(id).await
    }

    pub async fn create(&self, race: NewRace) -> Result<Race> {
        self.repo.create(race).await
    }

    pub async fn update(&self, race: Race) -> Result<Race> {
        self.repo.update(race).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await
    }
}
