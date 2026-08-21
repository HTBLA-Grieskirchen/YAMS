use poem_openapi::OpenApiService;
use yams_api::{Api, UnimplementedApi};

fn main() {
    let api_service = openapi_service(UnimplementedApi, []);
    println!("{}", api_service.spec());
}
