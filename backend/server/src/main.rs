use clap::Parser;
use poem::{EndpointExt, IntoResponse, Response};
use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use std::sync::Arc;
use yams_core::context::YamsContext;
use yams_core::services::{
    AddressService, AnimalService, ClientService, EventService, RaceService, SeminarService,
};
use yams_persistence::adapter::SqliteAdapter;

mod api;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Config {
    /// IP address to bind to
    #[arg(long, env = "BIND_ADDRESS", default_value = "127.0.0.1")]
    bind_address: String,

    /// Port to bind to
    #[arg(long, env = "PORT", default_value = "3000")]
    port: u16,

    /// Subpath this service is hosted on
    #[arg(long, env = "SUBPATH", default_value = "/")]
    subpath: String,

    /// Database URL
    #[arg(long, env = "DATABASE_URL", default_value = "yams.db")]
    database_url: String,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = Config::parse();

    let adapter = SqliteAdapter::new(&config.database_url)
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

    let base_path = config.subpath.trim_matches('/');
    let subpath = if base_path.is_empty() {
        "/".to_string()
    } else {
        format!("/{}/", base_path)
    };

    let api_url = format!(
        "http://{}:{}{}api",
        config.bind_address, config.port, subpath
    );

    // TODO: add dynamic version loading
    let api_service = OpenApiService::new(api::Api { ctx }, "YAMS API", env!("CARGO_PKG_VERSION"))
        .server("http://localhost:3000/api");

    let app = Route::new()
        .nest("/swagger", api_service.swagger_ui())
        .nest("/redoc", api_service.redoc())
        .nest("/spec.json", api_service.spec_endpoint())
        .nest("/spec.yaml", api_service.spec_endpoint_yaml().after(|res| async move { 
            res.map(IntoResponse::into_response).map(|mut resp| {
                resp.headers_mut().remove("content-disposition");
                resp
            })
        }))
        .nest("/api", api_service);

    println!("Server started at {}", api_url);
    Server::new(TcpListener::bind(format!(
        "{}:{}",
        config.bind_address, config.port
    )))
    .run(app)
    .await
}
