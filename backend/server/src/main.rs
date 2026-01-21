use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;
use std::fs::File;
use std::io::Write;

mod api;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "yams.db".to_string());
    let adapter = yams_persistence::adapter::SqliteAdapter::new(&db_url).await
        .expect("Failed to initialize database");
    let adapter = std::sync::Arc::new(adapter);

    let api_service = OpenApiService::new(api::Api { adapter }, "YAMS API", "1.0")
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
