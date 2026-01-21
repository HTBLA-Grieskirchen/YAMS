use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use yams_core::context::YamsContext;
use yams_core::services::{AddressService, ClientService, AnimalService, RaceService, EventService, SeminarService};
use yams_persistence::adapter::SqliteAdapter;

mod api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "yams.db".to_string());
    let adapter = SqliteAdapter::new(&db_url).await
        .expect("Failed to initialize database");
    let adapter = Arc::new(adapter);

    let ctx = Arc::new(YamsContext::new(
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
    ));

    let address_service = AddressService::new(ctx.clone());
    let client_service = ClientService::new(ctx.clone());
    let animal_service = AnimalService::new(ctx.clone());
    let race_service = RaceService::new(ctx.clone());
    let event_service = EventService::new(ctx.clone());
    let seminar_service = SeminarService::new(ctx.clone());

    let api_service = OpenApiService::new(
        api::Api { 
            address_service,
            client_service,
            animal_service,
            race_service,
            event_service,
            seminar_service,
        }, 
        "YAMS API", 
        "1.0"
    )
    .server("http://localhost:3000/api");

    if std::env::args().any(|arg| arg == "--export-spec") {
        let spec = api_service.spec();
        let mut file = File::create("openapi.json")?;
        file.write_all(spec.as_bytes())?;
        println!("OpenAPI spec exported to openapi.json");
        return Ok(());
    }

    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/docs", ui);

    println!("Server started at http://localhost:3000/docs");
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
