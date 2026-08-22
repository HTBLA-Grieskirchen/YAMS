use yams_core::ports::Clock;
use chrono::{DateTime, Duration, NaiveDate, Utc};

pub struct FixedClock {
    current_time: DateTime<Utc>,
}

impl FixedClock {
    pub fn new(current_time: DateTime<Utc>) -> Self {
        Self { current_time }
    }

    pub fn advance_by(&mut self, duration: Duration) {
        self.current_time += duration;
    }

    pub fn set_time(&mut self, time: DateTime<Utc>) {
        self.current_time = time;
    }
}

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        self.current_time
    }

    fn today(&self) -> NaiveDate {
        self.current_time.date_naive()
    }
}