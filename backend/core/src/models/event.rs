use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Option<Uuid>,
    pub date: DateTime<Utc>,
    pub location_id: Uuid,
    pub location_name: Option<String>,
    pub max_participants: Option<i32>,
    pub seminar_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seminar {
    pub id: Option<Uuid>,
    pub title: String,
    pub price: Decimal,
    pub duration: Option<Duration>,
}
