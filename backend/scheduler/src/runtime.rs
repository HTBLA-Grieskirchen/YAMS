use std::future::Future;
use std::pin::Pin;

use chrono_tz::Tz;
use scheduler::{
    InMemoryStateStore, Scheduler, SchedulerConfig, SchedulerError, SchedulerReport, StateStore,
};
use tracing::info;
use yams_core::App;

use crate::jobs::{TAGESABSCHLUSS_CRON, run};
use crate::observer::TracingObserver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct YamsSchedulerConfig {
    pub timezone: Tz,
}

impl Default for YamsSchedulerConfig {
    fn default() -> Self {
        Self {
            timezone: chrono_tz::Europe::Vienna,
        }
    }
}

pub struct YamsScheduler<S: StateStore = InMemoryStateStore> {
    app: App,
    store: S,
}

impl YamsScheduler {
    pub fn new(app: App) -> Self {
        Self {
            app,
            store: InMemoryStateStore::new(),
        }
    }
}

impl<S: StateStore + Send + Sync + 'static> YamsScheduler<S> {
    pub fn state_store<SS: StateStore>(self, store: SS) -> YamsScheduler<SS> {
        YamsScheduler {
            app: self.app,
            store,
        }
    }

    pub fn start(
        self,
        config: YamsSchedulerConfig,
    ) -> Pin<Box<dyn Future<Output = Result<SchedulerReport, SchedulerError>> + Send>> {
        let scheduler_config = SchedulerConfig {
            timezone: config.timezone,
            ..SchedulerConfig::default()
        };

        let scheduler = Scheduler::with_observer(scheduler_config, self.store, TracingObserver);
        info!(
            timezone = %config.timezone,
            cron = TAGESABSCHLUSS_CRON,
            "starting yams scheduler"
        );

        run(scheduler, self.app)
    }
}
