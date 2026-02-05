use poem_openapi::{NewType, Object};
use serde::{Deserialize, Serialize};
use yams_core::domain;

#[derive(NewType, Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
pub struct CountryCode(pub String);

#[derive(Object, Serialize, Deserialize, Debug, Clone)]
#[oai(rename_all = "camelCase")]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub postal_code: String,
    pub city: String,
    pub street_and_number: String,
    pub country_code: CountryCode,
}

impl From<Address> for domain::Address {
    fn from(value: Address) -> Self {
        Self {
            postal_code: value.postal_code,
            city: value.city,
            street_and_number: value.street_and_number,
            country_code: value.country_code.0,
        }
    }
}

impl From<domain::Address> for Address {
    fn from(value: domain::Address) -> Self {
        Self {
            postal_code: value.postal_code,
            city: value.city,
            street_and_number: value.street_and_number,
            country_code: CountryCode(value.country_code),
        }
    }
}
