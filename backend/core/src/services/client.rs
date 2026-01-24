use crate::error::Result;
use crate::models::{Client, NewClient};
use crate::ports::ClientRepository;
use std::sync::Arc;

pub struct ClientService {
    repo: Arc<dyn ClientRepository>,
}

impl ClientService {
    pub fn new(repo: Arc<dyn ClientRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Client>> {
        self.repo.find_all().await
    }

    pub async fn create(&self, client: NewClient) -> Result<Client> {
        self.repo.create(client).await
    }

    pub async fn update(&self, client: Client) -> Result<Client> {
        self.repo.update(client).await
    }
}
