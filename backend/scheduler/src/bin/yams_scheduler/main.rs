mod config;
mod tracing_setup;

use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use error_stack::{Report, ResultExt};
use tracing::info;
use yams_core::App;
use yams_filesystemstore::FileSystemObjectStore;
use yams_scheduler::{SQLiteStateStore, YamsScheduler, YamsSchedulerConfig};
use yams_sqlite::SQLiteInstance;
use yams_typstreports::TypstPdfRenderer;

use crate::config::{CliOverlay, resolve};
use crate::tracing_setup::init_tracing;

#[derive(Debug, Parser)]
#[command(name = "yams-scheduler", about = "YAMS background scheduler")]
struct Cli {
    #[arg(long = "config-path", env = "YAMS_SCHEDULER_CONFIG_PATH")]
    config_path: Option<PathBuf>,

    #[arg(long, env = "YAMS_DATABASE_URL")]
    database_url: Option<String>,

    #[arg(long, env = "YAMS_OBJECT_STORE_DIR")]
    object_store_dir: Option<PathBuf>,

    #[arg(long, env = "YAMS_LOG_DIR")]
    log_dir: Option<PathBuf>,

    #[arg(long, env = "YAMS_SCHEDULER_TIMEZONE")]
    timezone: Option<String>,
}

#[derive(Debug, thiserror::Error)]
#[error("yams-scheduler failed")]
struct SchedulerMainError;

#[tokio::main]
async fn main() -> Result<(), Report<SchedulerMainError>> {
    let cli = Cli::parse();
    let config = resolve(CliOverlay {
        config_path: cli.config_path,
        database_url: cli.database_url,
        object_store_dir: cli.object_store_dir,
        log_dir: cli.log_dir,
        timezone: cli.timezone,
    })
    .change_context(SchedulerMainError)
    .attach_opaque("config resolution")?;
    init_tracing(config.log_dir.as_deref());

    let sqlite = Arc::new(
        SQLiteInstance::local(&config.database_url)
            .await
            .change_context(SchedulerMainError)
            .attach_opaque("sqlite instance creation")?,
    );
    sqlite
        .migrate_repos_to_latest()
        .await
        .change_context(SchedulerMainError)
        .attach_opaque("sqlite repository migration")?;

    let object_store = FileSystemObjectStore::new(&config.object_store_dir)
        .change_context(SchedulerMainError)
        .attach_opaque("file system object store initialization")?;
    let app = App::builder()
        .uow_provider(Box::new(sqlite.clone()))
        .object_store(Arc::new(object_store))
        .pdf_renderer(Arc::new(TypstPdfRenderer::new()))
        .build();

    let mut store = SQLiteStateStore::new(sqlite);
    store
        .migrate_to_latest()
        .await
        .change_context(SchedulerMainError)
        .attach_opaque("sqlite job state store migration")?;

    let join = tokio::spawn(YamsScheduler::new(app).state_store(store).start(
        YamsSchedulerConfig {
            timezone: config.timezone,
        },
    ));

    info!("yams-scheduler running");
    tokio::signal::ctrl_c()
        .await
        .change_context(SchedulerMainError)
        .attach_opaque("ctrl+c signal handling")?;
    join.abort();
    let _ = join.await;
    info!("yams-scheduler stopped");
    Ok(())
}
