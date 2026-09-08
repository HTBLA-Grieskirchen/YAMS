use std::sync::Arc;

use chrono_tz::Tz;
use scheduler::{Scheduler, SchedulerConfig, SchedulerError, SchedulerReport};
use error_stack::{Report, ResultExt};
use tracing::info;
use yams_core::App;

use crate::errors::YamsSchedulerStartError;
use crate::jobs::build_tagesabschluss_job;
use crate::sqlite_state_store::SQLiteStateStore;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", default)]
pub struct YamsSchedulerConfig {
    pub enabled: bool,
    pub timezone: String,
    pub tagesabschluss_cron: String,
}

impl Default for YamsSchedulerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            timezone: "Europe/Vienna".into(),
            tagesabschluss_cron: "0 23 * * *".into(),
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

    let job = build_tagesabschluss_job(app, &config.tagesabschluss_cron)
        .map_err(|_| Report::new(YamsSchedulerStartError::Cron))?;

    let scheduler = Scheduler::with_log_observer(scheduler_config, store);
    info!(
        timezone = %config.timezone,
        cron = %config.tagesabschluss_cron,
        "starting yams scheduler"
    );

    let join = tokio::spawn(async move { scheduler.run(job).await });

    Ok(YamsSchedulerHandle { join })
}
