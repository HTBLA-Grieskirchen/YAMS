mod config;
mod tracing_setup;

use std::sync::Arc;

use error_stack::{Report, ResultExt};
use poem::http::StatusCode;
use poem::middleware::{CatchPanic, Compression, Cors, Middleware, RequestId, ReuseId, Tracing};
use poem::{EndpointExt, IntoResponse, Route, Server, listener::TcpListener};
use poem_openapi::payload::PlainText;
use thiserror::Error;
use tracing_setup::init_tracing;
use yams_api::{errors::InternalServerError, openapi_service};
use yams_core::App;
use yams_filesystemstore::FileSystemObjectStore;
use yams_scheduler::{SQLiteStateStore, start};
use yams_sqlite::{SQLiteInstance, SharedSQLiteInstance};
use yams_typstreports::TypstPdfRenderer;

#[derive(Debug, Error)]
#[error("Backend server fatal error")]
pub struct BackendServerError;

fn catch_panic() -> CatchPanic<impl poem::middleware::PanicHandler> {
    CatchPanic::new().with_handler(|err| {
        tracing::error!("Panic: {:?}", dbg!(err));
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            PlainText(InternalServerError),
        )
            .into_response()
    })
}

#[tokio::main]
async fn main() -> Result<(), Report<BackendServerError>> {
    let config = config::load().change_context(BackendServerError)?;
    init_tracing(config.log_dir.as_deref());

    let sqlite = Arc::new(
        SQLiteInstance::local(&config.database_url)
            .await
            .change_context(BackendServerError)?,
    );
    sqlite
        .migrate_repos_to_latest()
        .await
        .change_context(BackendServerError)?;

    let object_store =
        FileSystemObjectStore::new(config.object_store_dir).change_context(BackendServerError)?;
    let object_store = Arc::new(object_store);
    let pdf_renderer = Arc::new(TypstPdfRenderer::new());

    let build_app = |sqlite: Arc<SQLiteInstance>| {
        App::builder()
            .uow_provider(Box::new(SharedSQLiteInstance::new(sqlite.clone())))
            .object_store(object_store.clone())
            .pdf_renderer(pdf_renderer.clone())
            .build()
    };

    let app = build_app(sqlite.clone());

    let scheduler_handle = if config.scheduler.enabled {
        let store = SQLiteStateStore::new(sqlite.clone());
        Some(
            start(Arc::new(build_app(sqlite.clone())), store, &config.scheduler)
                .await
                .change_context(BackendServerError)?,
        )
    } else {
        None
    };

    let base_path = config.subpath.trim_matches('/');
    let subpath = if base_path.is_empty() {
        "/".to_string()
    } else {
        format!("/{}/", base_path)
    };

    let api_url = format!(
        "http://{}:{}{}api",
        config.bind_address, config.bind_port, subpath
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

    let tracing = Tracing.combine(RequestId::new().reuse_id(ReuseId::Use));

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
        .with(Compression::new())
        .with(tracing)
        .with(catch_panic())
        .with(cors);

    tracing::info!("Server started at {}", api_url);
    let result = Server::new(TcpListener::bind(format!(
        "{}:{}",
        config.bind_address, config.bind_port
    )))
    .run(app)
    .await
    .change_context(BackendServerError);

    if let Some(handle) = scheduler_handle {
        handle.shutdown().await;
    }

    result?;
    Ok(())
}
