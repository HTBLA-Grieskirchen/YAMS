use std::path::Path;
use std::sync::OnceLock;

use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::LogTarget;

static LOG_GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();

pub fn init_tracing(target: LogTarget, log_dir: &Path) {
    if matches!(target, LogTarget::Disabled) {
        return;
    }

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("YAMS_LOG_LEVEL")
        .from_env_lossy();

    match target {
        LogTarget::Disabled => {}
        LogTarget::Stdout => {
            let stdout_layer = fmt::layer()
                .with_writer(std::io::stdout)
                .with_filter(filter);
            tracing_subscriber::registry()
                .with(stdout_layer)
                .init();
        }
        LogTarget::File => {
            let json_layer = json_file_layer(log_dir, filter);
            tracing_subscriber::registry().with(json_layer).init();
        }
        LogTarget::Both => {
            let stdout_layer = fmt::layer()
                .with_writer(std::io::stdout)
                .with_filter(filter.clone());
            let json_layer = json_file_layer(log_dir, filter);
            tracing_subscriber::registry()
                .with(stdout_layer)
                .with(json_layer)
                .init();
        }
    }
}

fn json_file_layer<S>(log_dir: &Path, filter: EnvFilter) -> impl Layer<S> + use<S>
where
    S: tracing::Subscriber,
    for<'a> S: tracing_subscriber::registry::LookupSpan<'a>,
{
    std::fs::create_dir_all(log_dir).expect("failed to create log directory");

    let file_appender = tracing_appender::rolling::daily(log_dir, "yams-server.json");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    LOG_GUARD.set(guard).ok();

    fmt::layer()
        .json()
        .with_writer(non_blocking)
        .with_filter(filter)
}
