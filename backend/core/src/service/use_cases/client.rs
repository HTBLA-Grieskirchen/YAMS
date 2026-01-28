use async_trait::async_trait;
use chrono::NaiveDate;

use super::UseCase;
use crate::{
    domain::{Address, Client, Email, MobileNumber, errors::NoError, factories::NewClient},
    ports::uow::UnitOfWork,
    service::{
        Registry, ServiceToUseCaseError, UseCaseError, UseCaseResult, errors::PersistenceError,
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

#[async_trait]
impl UseCase<Client> for CreateClient {
    type DomainError = NoError;
    type ServiceError = PersistenceError;

    async fn perform(
        self,
        registry: &mut Registry,
    ) -> UseCaseResult<Client, Self::DomainError, Self::ServiceError> {
        let result = registry
            .uow
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

        registry
            .uow
            .commit()
            .await
            .map_err(ServiceToUseCaseError::into_service)?;

        match result {
            Ok(client) => Ok(client.into_data()),
            Err(e) => Err(UseCaseError::Service(e)),
        }
    }
}
