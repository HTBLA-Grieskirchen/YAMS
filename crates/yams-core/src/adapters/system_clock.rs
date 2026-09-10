use chrono::{DateTime, NaiveDate, Utc};
use tracing::trace;

use crate::ports::Clock;

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        let now = Utc::now();
        trace!(%now, "system clock now");
        now
    }

    fn today(&self) -> NaiveDate {
        let today = Utc::now().date_naive();
        trace!(%today, "system clock today");
        today
    }
}
