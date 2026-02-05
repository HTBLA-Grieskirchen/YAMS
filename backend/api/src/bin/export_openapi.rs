use poem_openapi::OpenApiService;
use yams_api::api::{Api, UnimplementedApi};

type SpecApi = Api<UnimplementedApi>;

fn main() {
    let api_service = OpenApiService::new(SpecApi::from(UnimplementedApi), "YAMS API", "1.0");

    println!("{}", api_service.spec());
}
