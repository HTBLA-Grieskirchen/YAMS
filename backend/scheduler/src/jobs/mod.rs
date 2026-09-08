mod tagesabschluss;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use scheduler::{Scheduler, SchedulerError, SchedulerReport, StateStore};
use yams_core::App;

pub use tagesabschluss::{TAGESABSCHLUSS_CRON, TAGESABSCHLUSS_JOB_ID, build_tagesabschluss_job};

pub fn run<S: StateStore + Send + Sync + 'static>(
    scheduler: Scheduler<S>,
    app: App,
) -> Pin<Box<dyn Future<Output = Result<SchedulerReport, SchedulerError>> + Send>> {
    let job = build_tagesabschluss_job(Arc::new(app));
    Box::pin(async move { scheduler.run(job).await })
}
