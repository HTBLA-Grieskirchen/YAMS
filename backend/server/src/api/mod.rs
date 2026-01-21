use poem_openapi::{OpenApi, payload::Json, Object, ApiResponse};
use std::sync::Arc;
use yams_core::models::Address;
use yams_core::ports::AddressRepository;
use yams_persistence::adapter::SqliteAdapter;
use uuid::Uuid;

pub struct Api {
    pub adapter: Arc<SqliteAdapter>,
}

#[derive(Object, Serialize, Deserialize)]
#[oai(rename_all = "camelCase")]
struct AddressDTO {
    pub id: Option<Uuid>,
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

use serde::{Deserialize, Serialize};

#[derive(ApiResponse)]
enum GetAddressesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<AddressDTO>>),
    #[oai(status = 500)]
    InternalError,
}

#[derive(ApiResponse)]
enum CreateAddressResponse {
    #[oai(status = 200)]
    Ok(Json<AddressDTO>),
    #[oai(status = 500)]
    InternalError,
}

#[OpenApi]
impl Api {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    #[oai(path = "/addresses", method = "get")]
    async fn get_addresses(&self) -> GetAddressesResponse {
        match AddressRepository::find_all(&*self.adapter).await {
            Ok(addresses) => GetAddressesResponse::Ok(Json(addresses.into_iter().map(AddressDTO::from).collect())),
            Err(_) => GetAddressesResponse::InternalError,
        }
    }

    #[oai(path = "/addresses", method = "post")]
    async fn create_address(&self, address: Json<AddressDTO>) -> CreateAddressResponse {
        let address: Address = address.0.into();
        match AddressRepository::save(&*self.adapter, address).await {
            Ok(saved) => CreateAddressResponse::Ok(Json(AddressDTO::from(saved))),
            Err(_) => CreateAddressResponse::InternalError,
        }
    }
}
