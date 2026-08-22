use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::uow::Versioned,
    domain::{Address, Client, Email, MobileNumber, factories::NewClient},
    service::{ExecutionContext, UseCase},
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
    #[error("error during creation of client")]
    Creation,
}

#[async_trait]
impl UseCase<Client> for CreateClient {
    type Error = Report<CreateClientError>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Client, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;

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

        result
            .map(Versioned::into_data)
            .map_err(IntoReport::into_report)
            .change_context(CreateClientError::Creation)
    }
}
