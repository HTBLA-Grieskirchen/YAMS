use chrono::{DateTime, NaiveDate, Utc};

pub trait Clock: Send + Sync {
    /// Returns the current date and time in the UTC timezone.
    fn now(&self) -> DateTime<Utc>;

    /// Returns the current date in the UTC timezone.
    fn today(&self) -> NaiveDate;
}
