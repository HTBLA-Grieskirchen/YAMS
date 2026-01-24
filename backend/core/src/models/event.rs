use chrono::{DateTime, Duration, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub date: DateTime<Utc>,
    pub location_id: Uuid,
    pub location_name: Option<String>,
    pub max_participants: Option<i32>,
    pub seminar_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewEvent {
    pub date: DateTime<Utc>,
    pub location_id: Uuid,
    pub location_name: Option<String>,
    pub max_participants: Option<i32>,
    pub seminar_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seminar {
    pub id: Uuid,
    pub title: String,
    pub price: Decimal,
    pub duration: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewSeminar {
    pub title: String,
    pub price: Decimal,
    pub duration: Option<Duration>,
}
