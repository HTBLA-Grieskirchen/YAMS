use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init_tracing() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .with_env_var("YAMS_LOG_LEVEL")
        .from_env_lossy();
    fmt::Subscriber::builder()
        .with_writer(std::io::stdout)
        .with_env_filter(filter)
        .init();
}
