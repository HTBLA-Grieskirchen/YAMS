use std::sync::Arc;
use crate::models::Client;
use crate::context::YamsContext;
use crate::error::Result;

pub struct ClientService {
    ctx: Arc<YamsContext>,
}

impl ClientService {
    pub fn new(ctx: Arc<YamsContext>) -> Self {
        Self { ctx }
    }

    pub async fn get_all(&self) -> Result<Vec<Client>> {
        self.ctx.client_repo.find_all().await
    }

    pub async fn create(&self, client: Client) -> Result<Client> {
        self.ctx.client_repo.save(client).await
    }
}
