mod config;

use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use error_stack::{Report, ResultExt};
use tracing::info;
use yams_core::App;
use yams_filesystemstore::FileSystemObjectStore;
use yams_scheduler::{SQLiteStateStore, start};
use yams_sqlite::{SQLiteInstance, SharedSQLiteInstance};
use yams_typstreports::TypstPdfRenderer;

#[derive(Debug, Parser)]
#[command(name = "yams-scheduler", about = "YAMS background scheduler")]
struct Cli {
    #[arg(long = "config-path", env = "YAMS_SCHEDULER_CONFIG_PATH")]
    config_path: Option<PathBuf>,
}

#[derive(Debug, thiserror::Error)]
#[error("yams-scheduler failed")]
struct SchedulerMainError;

#[tokio::main]
async fn main() -> Result<(), Report<SchedulerMainError>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let config = config::load(cli.config_path).change_context(SchedulerMainError)?;

    let sqlite = Arc::new(
        SQLiteInstance::local(&config.database_url)
            .await
            .change_context(SchedulerMainError)?,
    );
    sqlite
        .migrate_repos_to_latest()
        .await
        .change_context(SchedulerMainError)?;

    let object_store =
        FileSystemObjectStore::new(&config.object_store_dir).change_context(SchedulerMainError)?;
    let app = Arc::new(
        App::builder()
            .uow_provider(Box::new(SharedSQLiteInstance::new(sqlite.clone())))
            .object_store(Arc::new(object_store))
            .pdf_renderer(Arc::new(TypstPdfRenderer::new()))
            .build(),
    );

    let store = SQLiteStateStore::new(sqlite);
    let handle = start(app, store, &config.scheduler)
        .await
        .change_context(SchedulerMainError)?;

    info!("yams-scheduler running");
    tokio::signal::ctrl_c()
        .await
        .change_context(SchedulerMainError)?;
    handle.shutdown().await;
    info!("yams-scheduler stopped");
    Ok(())
}
