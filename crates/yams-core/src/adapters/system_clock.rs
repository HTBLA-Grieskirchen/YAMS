use chrono::{DateTime, NaiveDate, Utc};

use crate::ports::Clock;

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn today(&self) -> NaiveDate {
        Utc::now().date_naive()
    }
}
