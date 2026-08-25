use std::sync::Mutex;

use chrono::{DateTime, Duration, NaiveDate, Utc};
use yams_core::ports::Clock;

pub struct FixedClock {
    current_time: Mutex<DateTime<Utc>>,
}

impl FixedClock {
    pub fn new(current_time: DateTime<Utc>) -> Self {
        Self {
            current_time: Mutex::new(current_time),
        }
    }

    pub fn advance_by(&self, duration: Duration) {
        let mut current_time = self.current_time.lock().unwrap();
        *current_time += duration;
    }

    pub fn set_time(&mut self, time: DateTime<Utc>) {
        let mut current_time = self.current_time.lock().unwrap();
        *current_time = time;
    }
}

impl Clock for FixedClock {
    fn now(&self) -> DateTime<Utc> {
        *self.current_time.lock().unwrap()
    }

    fn today(&self) -> NaiveDate {
        self.current_time.lock().unwrap().date_naive()
    }
}
