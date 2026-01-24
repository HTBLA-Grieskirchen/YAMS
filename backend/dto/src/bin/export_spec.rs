use poem_openapi::OpenApi;
use poem_openapi::OpenApiService;
use poem_openapi::payload::Json;
use yams_dto::{CreateAddressResponse, GetAddressesResponse, NewAddressDTO};

struct SpecApi;

#[OpenApi]
impl SpecApi {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    #[oai(path = "/addresses", method = "get")]
    async fn get_addresses(&self) -> GetAddressesResponse {
        unimplemented!()
    }

    #[oai(path = "/addresses", method = "post")]
    async fn create_address(&self, _address: Json<NewAddressDTO>) -> CreateAddressResponse {
        unimplemented!()
    }
}

fn main() {
    let api_service =
        OpenApiService::new(SpecApi, "YAMS API", "1.0").server("http://localhost:3000/api");
    println!("{}", api_service.spec());
}
