use chrono::NaiveDate;
use yams_core::service::CreateClient;

use crate::schema::Address;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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

impl From<ClientCreation> for CreateClient {
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
