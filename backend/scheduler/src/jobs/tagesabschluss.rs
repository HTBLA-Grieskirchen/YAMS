use std::sync::Arc;

use scheduler::{CronSchedule, Job, Schedule, Task, TaskContext};
use tracing::{error, info};
use yams_core::App;
use yams_core::service::TagesabschlussDurchführen;

pub const TAGESABSCHLUSS_JOB_ID: &str = "tagesabschluss";

/// Daily at 23:00 in `SchedulerConfig::timezone` (business requirement, not configurable).
pub const TAGESABSCHLUSS_CRON: &str = "0 23 * * *";

pub fn build_tagesabschluss_job(app: Arc<App>) -> Job<Arc<App>> {
    let cron = CronSchedule::parse(TAGESABSCHLUSS_CRON)
        .expect("hardcoded tagesabschluss cron expression must be valid");
    Job::new(
        TAGESABSCHLUSS_JOB_ID,
        Schedule::Cron(cron),
        app,
        Task::from_async(|context: TaskContext<Arc<App>>| async move {
            let abschlussdatum = context
                .run
                .scheduled_at
                .with_timezone(&context.run.timezone)
                .date_naive();
            info!(%abschlussdatum, "scheduled tagesabschluss started");
            match context
                .deps
                .execute(TagesabschlussDurchführen {
                    abschlussdatum: Some(abschlussdatum),
                })
                .await
            {
                Ok(rechnungen) => {
                    info!(
                        count = rechnungen.len(),
                        "scheduled tagesabschluss finished"
                    );
                    Ok(())
                }
                Err(err) => {
                    error!(?err, "scheduled tagesabschluss failed");
                    Err(err.to_string())
                }
            }
        }),
    )
}
