use poem_openapi::{OpenApi, payload::Json};
use yams_dto::{AddressDTO, GetAddressesResponse, CreateAddressResponse};
use yams_core::models::Address;
use yams_core::services::AddressService;

pub struct Api {
    pub address_service: AddressService,
}

#[OpenApi]
impl Api {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    #[oai(path = "/addresses", method = "get")]
    async fn get_addresses(&self) -> GetAddressesResponse {
        match self.address_service.get_all().await {
            Ok(addresses) => GetAddressesResponse::Ok(Json(addresses.into_iter().map(AddressDTO::from).collect())),
            Err(_) => GetAddressesResponse::InternalError,
        }
    }

    #[oai(path = "/addresses", method = "post")]
    async fn create_address(&self, address: Json<AddressDTO>) -> CreateAddressResponse {
        let address: Address = address.0.into();
        match self.address_service.create(address).await {
            Ok(saved) => CreateAddressResponse::Ok(Json(AddressDTO::from(saved))),
            Err(_) => CreateAddressResponse::InternalError,
        }
    }
}
