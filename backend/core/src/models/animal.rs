use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animal {
    pub id: Uuid,
    pub name: String,
    pub birthdate: DateTime<Utc>,
    pub race_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewAnimal {
    pub name: String,
    pub birthdate: DateTime<Utc>,
    pub race_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Race {
    pub id: Uuid,
    pub description: String,
    pub animal_species: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewRace {
    pub description: String,
    pub animal_species: String,
}
