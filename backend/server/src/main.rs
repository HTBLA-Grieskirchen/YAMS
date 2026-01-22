use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use yams_core::context::YamsContext;
use yams_core::services::{
    AddressService, AnimalService, ClientService, EventService, RaceService, SeminarService,
};
use yams_persistence::adapter::SqliteAdapter;

mod api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "yams.db".to_string());
    let adapter = SqliteAdapter::new(&db_url)
        .await
        .expect("Failed to initialize database");
    let adapter = Arc::new(adapter);

    let address_service = Arc::new(AddressService::new(adapter.clone()));
    let client_service = Arc::new(ClientService::new(adapter.clone()));
    let animal_service = Arc::new(AnimalService::new(adapter.clone()));
    let race_service = Arc::new(RaceService::new(adapter.clone()));
    let event_service = Arc::new(EventService::new(adapter.clone()));
    let seminar_service = Arc::new(SeminarService::new(adapter.clone()));

    let ctx = Arc::new(YamsContext::new(
        address_service,
        client_service,
        animal_service,
        race_service,
        event_service,
        seminar_service,
    ));

    // TODO: add dynamic version loading
    let api_service = OpenApiService::new(api::Api { ctx }, "YAMS API", "1.0")
        .server("http://localhost:3000/api");

    let app = Route::new()
        .nest("/swagger", api_service.swagger_ui())
        .nest("/redoc", api_service.redoc())
        .nest("/spec.json", api_service.spec_endpoint())
        .nest("/sepc.yaml", api_service.spec_endpoint_yaml())
        .nest("/api", api_service);

    println!("Server started at http://localhost:3000/api");
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
