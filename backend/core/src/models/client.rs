use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: DateTime<Utc>,
    pub email: String,
    pub mobile_number: String,
    pub customer_number: i64,
    pub address_id: Uuid,
    pub consent: bool,
    pub animal_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewClient {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: DateTime<Utc>,
    pub email: String,
    pub mobile_number: String,
    pub customer_number: i64,
    pub address_id: Uuid,
    pub consent: bool,
    pub animal_ids: Vec<Uuid>,
}
