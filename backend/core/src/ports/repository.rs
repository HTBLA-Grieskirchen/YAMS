use async_trait::async_trait;
use uuid::Uuid;
use crate::models::*;

#[async_trait]
pub trait ClientRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Client>, crate::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Client>, crate::Error>;
    async fn create(&self, client: NewClient) -> Result<Client, crate::Error>;
    async fn update(&self, client: Client) -> Result<Client, crate::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), crate::Error>;
}

#[async_trait]
pub trait AddressRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Address>, crate::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Address>, crate::Error>;
    async fn create(&self, address: NewAddress) -> Result<Address, crate::Error>;
    async fn update(&self, address: Address) -> Result<Address, crate::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), crate::Error>;
}

#[async_trait]
pub trait AnimalRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Animal>, crate::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Animal>, crate::Error>;
    async fn create(&self, animal: NewAnimal) -> Result<Animal, crate::Error>;
    async fn update(&self, animal: Animal) -> Result<Animal, crate::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), crate::Error>;
}

#[async_trait]
pub trait RaceRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Race>, crate::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Race>, crate::Error>;
    async fn create(&self, race: NewRace) -> Result<Race, crate::Error>;
    async fn update(&self, race: Race) -> Result<Race, crate::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), crate::Error>;
}

#[async_trait]
pub trait EventRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Event>, crate::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Event>, crate::Error>;
    async fn create(&self, event: NewEvent) -> Result<Event, crate::Error>;
    async fn update(&self, event: Event) -> Result<Event, crate::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), crate::Error>;
}

#[async_trait]
pub trait SeminarRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Seminar>, crate::Error>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Seminar>, crate::Error>;
    async fn create(&self, seminar: NewSeminar) -> Result<Seminar, crate::Error>;
    async fn update(&self, seminar: Seminar) -> Result<Seminar, crate::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), crate::Error>;
}
