use std::sync::Arc;

use chrono_tz::Tz;
use error_stack::{Report, ResultExt};
use scheduler::{Scheduler, SchedulerConfig, SchedulerError, SchedulerReport};
use tracing::info;
use yams_core::App;

use crate::errors::YamsSchedulerStartError;
use crate::jobs::{TAGESABSCHLUSS_CRON, build_tagesabschluss_job};
use crate::sqlite_state_store::SQLiteStateStore;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct YamsSchedulerConfig {
    pub enabled: bool,
    pub timezone: String,
}

impl Default for YamsSchedulerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            timezone: "Europe/Vienna".into(),
        }
    }
}

pub struct YamsSchedulerHandle {
    join: tokio::task::JoinHandle<Result<SchedulerReport, SchedulerError>>,
}

impl YamsSchedulerHandle {
    pub async fn shutdown(self) {
        self.join.abort();
        let _ = self.join.await;
    }
}

pub async fn start(
    app: Arc<App>,
    mut store: SQLiteStateStore,
    config: &YamsSchedulerConfig,
) -> Result<YamsSchedulerHandle, Report<YamsSchedulerStartError>> {
    if !config.enabled {
        return Err(Report::new(YamsSchedulerStartError::Start));
    }

    store
        .migrate_to_latest()
        .await
        .change_context(YamsSchedulerStartError::Migration)?;

    let timezone = config
        .timezone
        .parse::<Tz>()
        .change_context(YamsSchedulerStartError::Timezone)?;

    let scheduler_config = SchedulerConfig {
        timezone,
        ..SchedulerConfig::default()
    };

    let job = build_tagesabschluss_job(app);

    let scheduler = Scheduler::with_log_observer(scheduler_config, store);
    info!(
        timezone = %config.timezone,
        cron = TAGESABSCHLUSS_CRON,
        "starting yams scheduler"
    );

    let join = tokio::spawn(async move { scheduler.run(job).await });

    Ok(YamsSchedulerHandle { join })
}
