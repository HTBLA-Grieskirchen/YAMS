use std::sync::Arc;
use uuid::Uuid;
use crate::models::{Animal, NewAnimal, Race, NewRace};
use crate::context::YamsContext;
use crate::error::Result;

pub struct AnimalService {
    ctx: Arc<YamsContext>,
}

impl AnimalService {
    pub fn new(ctx: Arc<YamsContext>) -> Self {
        Self { ctx }
    }

    pub async fn get_all(&self) -> Result<Vec<Animal>> {
        self.ctx.animal_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Animal>> {
        self.ctx.animal_repo.find_by_id(id).await
    }

    pub async fn create(&self, animal: NewAnimal) -> Result<Animal> {
        self.ctx.animal_repo.create(animal).await
    }

    pub async fn update(&self, animal: Animal) -> Result<Animal> {
        self.ctx.animal_repo.update(animal).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.ctx.animal_repo.delete(id).await
    }
}

pub struct RaceService {
    ctx: Arc<YamsContext>,
}

impl RaceService {
    pub fn new(ctx: Arc<YamsContext>) -> Self {
        Self { ctx }
    }

    pub async fn get_all(&self) -> Result<Vec<Race>> {
        self.ctx.race_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Race>> {
        self.ctx.race_repo.find_by_id(id).await
    }

    pub async fn create(&self, race: NewRace) -> Result<Race> {
        self.ctx.race_repo.create(race).await
    }

    pub async fn update(&self, race: Race) -> Result<Race> {
        self.ctx.race_repo.update(race).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.ctx.race_repo.delete(id).await
    }
}
