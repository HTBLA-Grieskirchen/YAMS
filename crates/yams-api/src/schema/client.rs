use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::domain;

use crate::schema::{Address, Animal, schema_animal_from_domain};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Client {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDate,
    pub email: String,
    pub mobile_number: String,
    pub customer_number: i64,
    pub consent: bool,
    pub address: Address,
    pub animals: Vec<Animal>,
}

pub fn schema_client_from_domain(client: domain::Client, animals: Vec<domain::Animal>) -> Client {
    Client {
        id: client.id.0,
        first_name: client.first_name,
        last_name: client.last_name,
        birthdate: client.birthdate,
        email: client.email.0,
        mobile_number: client.mobile_number.0,
        customer_number: client.customer_number,
        consent: client.consent,
        address: client.address.into(),
        animals: animals.into_iter().map(schema_animal_from_domain).collect(),
    }
}
