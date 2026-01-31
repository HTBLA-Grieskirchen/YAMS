use async_trait::async_trait;
use chrono::NaiveDate;

use super::UseCase;
use crate::{
    application::OrchestratableError,
    domain::{Address, Client, Email, MobileNumber, factories::NewClient},
    ports::repos::Versioned,
    service::{
        Registry,
        errors::PersistenceError,
    },
};

#[derive(Clone)]
pub struct CreateClient {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDate,
    pub email: Email,
    pub mobile_number: MobileNumber,
    pub customer_number: i64,
    pub consent: bool,
    pub address: Address,
}

#[derive(thiserror::Error, Debug)]
pub enum CreateClientError {
    #[error(transparent)]
    Persistence(#[from] PersistenceError),
}

impl OrchestratableError for CreateClientError {
    fn should_retry(&self) -> bool {
        match self {
            CreateClientError::Persistence(e) => e.should_retry(),
        }
    }
}

#[async_trait]
impl UseCase<Client> for CreateClient {
    type Error = CreateClientError;

    async fn perform(
        self,
        registry: &mut Registry,
    ) -> Result<Client, Self::Error> {
        let Registry { uow, .. } = registry;

        let result = uow
            .clients()
            .create(NewClient {
                first_name: self.first_name,
                last_name: self.last_name,
                birthdate: self.birthdate,
                email: self.email,
                mobile_number: self.mobile_number,
                customer_number: self.customer_number,
                consent: self.consent,
                address: self.address,
            })
            .await;

        Ok(result.map(Versioned::into_data)?)
    }
}
