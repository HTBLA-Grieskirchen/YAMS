use clap::Parser;
use error_stack::{Report, ResultExt};
use poem::middleware::Cors;
use poem::{EndpointExt, IntoResponse};
use poem::{Route, Server, listener::TcpListener};
use thiserror::Error;
use yams_api::openapi_service;
use yams_core::App;
use yams_persistence::SQLiteInstance;

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

#[derive(Debug, Error)]
#[error("Backend server fatal error")]
pub struct BackendServerError;

#[tokio::main]
async fn main() -> Result<(), Report<BackendServerError>> {
    let config = Config::parse();

    let mut adapter = SQLiteInstance::local(&config.database_url)
        .await
        .expect("Failed to initialize database");
    adapter
        .migrate_to_latest()
        .await
        .change_context(BackendServerError)?;

    let app = App::builder().uow_provider(Box::new(adapter)).build();

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
    let api_service = openapi_service(app, [api_url.clone()]);

    let cors = Cors::new().allow_origins_fn(|origin| {
        origin.starts_with("http://localhost:")
            || origin.starts_with("http://127.0.0.1:")
            || origin.starts_with("https://localhost:")
            || origin.starts_with("https://127.0.0.1:")
            || origin == "tauri://localhost"
    });

    let app = Route::new()
        .nest("/swagger", api_service.swagger_ui())
        .nest("/redoc", api_service.redoc())
        .nest("/spec.json", api_service.spec_endpoint())
        .nest(
            "/spec.yaml",
            api_service.spec_endpoint_yaml().after(|res| async move {
                res.map(IntoResponse::into_response).map(|mut resp| {
                    resp.headers_mut().remove("content-disposition");
                    resp
                })
            }),
        )
        .nest("/api", api_service)
        .with(cors);

    println!("Server started at {}", api_url);
    Server::new(TcpListener::bind(format!(
        "{}:{}",
        config.bind_address, config.port
    )))
    .run(app)
    .await
    .change_context(BackendServerError)?;
    Ok(())
}
