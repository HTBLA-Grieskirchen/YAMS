use async_trait::async_trait;
use yams_core::{domain::{Client, ClientId, factories::NewClient}, ports::repos::{ClientRepository, RepositoryResult, Versioned}};

pub struct SQLiteClientRepository {

}

#[async_trait]
impl ClientRepository for SQLiteClientRepository {
    async fn find_by_id(&self, id: ClientId) -> RepositoryResult<Option<Versioned<Client>>> {
        todo!()
    }
    
    async fn create(&self, client: NewClient) -> RepositoryResult<Versioned<Client>> {
        todo!()
    }

    async fn update(&self, client: &mut Versioned<Client>) -> RepositoryResult<()> {
        todo!()
    }

    async fn delete(&self, client: Versioned<Client>) -> RepositoryResult<()> {
        todo!()
    }
}