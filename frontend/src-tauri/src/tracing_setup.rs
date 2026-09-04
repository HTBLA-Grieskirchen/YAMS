use std::path::Path;
use std::sync::OnceLock;

use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

static LOG_GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();

pub fn init_tracing(log_dir: &Path) {
    std::fs::create_dir_all(log_dir).expect("failed to create log directory");

    let file_appender = tracing_appender::rolling::daily(log_dir, "yams.json");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    LOG_GUARD.set(guard).ok();

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("YAMS_LOG_LEVEL")
        .from_env_lossy();

    let stdout_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(filter.clone());

    let json_layer = fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_filter(filter);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(json_layer)
        .init();
}
