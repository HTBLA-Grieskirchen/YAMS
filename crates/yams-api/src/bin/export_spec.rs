use yams_api::{api::UnimplementedApi, openapi_service};

fn main() {
    let api_service = openapi_service(UnimplementedApi, std::iter::empty::<String>());
    println!("{}", api_service.spec());
}
