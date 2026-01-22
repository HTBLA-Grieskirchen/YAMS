use std::sync::Arc;
use uuid::Uuid;
use crate::models::{Address, NewAddress};
use crate::ports::AddressRepository;
use crate::error::Result;

pub struct AddressService {
    repo: Arc<dyn AddressRepository>,
}

impl AddressService {
    pub fn new(repo: Arc<dyn AddressRepository>) -> Self {
        Self { repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Address>> {
        self.repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Address>> {
        self.repo.find_by_id(id).await
    }

    pub async fn create(&self, address: NewAddress) -> Result<Address> {
        self.repo.create(address).await
    }

    pub async fn update(&self, address: Address) -> Result<Address> {
        self.repo.update(address).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete(id).await
    }
}
