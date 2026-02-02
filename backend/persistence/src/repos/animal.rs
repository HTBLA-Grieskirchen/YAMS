use async_trait::async_trait;
use yams_core::{domain::{Animal, AnimalId, factories::NewAnimal}, ports::repos::{AnimalRepository, RepositoryResult, Versioned}};

pub struct SQLiteAnimalRepository {

}

#[async_trait]
impl AnimalRepository for SQLiteAnimalRepository {
    async fn find_by_id(&self, id: AnimalId) -> RepositoryResult<Option<Versioned<Animal>>> {
        todo!()
    }

    async fn create(&self, animal: NewAnimal) -> RepositoryResult<Versioned<Animal>> {
        todo!()
    }

    async fn update(&self, animal: &mut Versioned<Animal>) -> RepositoryResult<()> {
        todo!()
    }

    async fn delete(&self, animal: Versioned<Animal>) -> RepositoryResult<()> {
        todo!()
    }
}
