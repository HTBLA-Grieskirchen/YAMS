use clap::Parser;
use poem::{EndpointExt, IntoResponse};
use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use std::sync::Arc;
use yams_core::App;
use yams_core::application::AppConfiguration;
use yams_persistence::SQLiteInstance;

use crate::api::{AppApi, AppApiImplementation};

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
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();

    let mut adapter = SQLiteInstance::local(&config.database_url)
        .await
        .expect("Failed to initialize database");
    adapter.migrate_to_latest().await?;

    let app = App {
        uow_provider: Box::new(adapter),
        configuration: AppConfiguration::default(),
    };

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
    let api_service = OpenApiService::new(
        AppApi::from(AppApiImplementation::new(app)),
        "YAMS API",
        env!("CARGO_PKG_VERSION"),
    )
    .server(&api_url);

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
        .nest("/api", api_service);

    println!("Server started at {}", api_url);
    Server::new(TcpListener::bind(format!(
        "{}:{}",
        config.bind_address, config.port
    )))
    .run(app)
    .await?;
    Ok(())
}
