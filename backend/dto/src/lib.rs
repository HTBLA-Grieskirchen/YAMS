use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yams_core::models::{Address, NewAddress};

#[derive(Object, Serialize, Deserialize, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct AddressDTO {
    pub id: Uuid,
    pub country: String,
    pub postal_code: String,
    pub city: String,
    pub street: String,
    pub street_number: String,
    pub extra: String,
}

#[derive(Object, Serialize, Deserialize, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct NewAddressDTO {
    pub country: String,
    pub postal_code: String,
    pub city: String,
    pub street: String,
    pub street_number: String,
    pub extra: String,
}

impl From<Address> for AddressDTO {
    fn from(a: Address) -> Self {
        Self {
            id: a.id,
            country: a.country,
            postal_code: a.postal_code,
            city: a.city,
            street: a.street,
            street_number: a.street_number,
            extra: a.extra,
        }
    }
}

impl From<AddressDTO> for Address {
    fn from(a: AddressDTO) -> Self {
        Self {
            id: a.id,
            country: a.country,
            postal_code: a.postal_code,
            city: a.city,
            street: a.street,
            street_number: a.street_number,
            extra: a.extra,
        }
    }
}

impl From<NewAddressDTO> for NewAddress {
    fn from(a: NewAddressDTO) -> Self {
        Self {
            country: a.country,
            postal_code: a.postal_code,
            city: a.city,
            street: a.street,
            street_number: a.street_number,
            extra: a.extra,
        }
    }
}

#[derive(ApiResponse)]
pub enum GetAddressesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<AddressDTO>>),
    #[oai(status = 500)]
    InternalError,
}

#[derive(ApiResponse)]
pub enum CreateAddressResponse {
    #[oai(status = 200)]
    Ok(Json<AddressDTO>),
    #[oai(status = 500)]
    InternalError,
}
