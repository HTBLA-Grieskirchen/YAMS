use crate::{
    Address, Client,
    errors::InternalServerError,
};
use chrono::NaiveDate;
use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::{Deserialize, Serialize};
use yams_core::use_cases;

#[derive(Object, Serialize, Deserialize, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct ClientCreation {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDate,
    pub email: String,
    pub mobile_number: String,
    pub customer_number: i64,
    pub consent: bool,
    pub address: Address,
}

impl From<ClientCreation> for use_cases::client::CreateClient {
    fn from(value: ClientCreation) -> Self {
        Self {
            first_name: value.first_name,
            last_name: value.last_name,
            birthdate: value.birthdate,
            email: value.email.into(),
            mobile_number: value.mobile_number.into(),
            customer_number: value.customer_number,
            consent: value.consent,
            address: value.address.into(),
        }
    }
}

#[derive(ApiResponse)]
pub enum CreateClientResponse {
    #[oai(status = 200)]
    Ok(Json<Client>),
    #[oai(status = 500)]
    InternalError(Json<InternalServerError>),
}
