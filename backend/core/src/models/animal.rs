use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animal {
    pub id: Option<Uuid>,
    pub name: String,
    pub birthdate: DateTime<Utc>,
    pub race_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Race {
    pub id: Option<Uuid>,
    pub description: String,
    pub animal_species: String,
}
