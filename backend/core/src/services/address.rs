use std::sync::Arc;
use uuid::Uuid;
use crate::models::{Address, NewAddress};
use crate::context::YamsContext;
use crate::error::Result;

pub struct AddressService {
    ctx: Arc<YamsContext>,
}

impl AddressService {
    pub fn new(ctx: Arc<YamsContext>) -> Self {
        Self { ctx }
    }

    pub async fn get_all(&self) -> Result<Vec<Address>> {
        self.ctx.address_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Address>> {
        self.ctx.address_repo.find_by_id(id).await
    }

    pub async fn create(&self, address: NewAddress) -> Result<Address> {
        self.ctx.address_repo.create(address).await
    }

    pub async fn update(&self, address: Address) -> Result<Address> {
        self.ctx.address_repo.update(address).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.ctx.address_repo.delete(id).await
    }
}
