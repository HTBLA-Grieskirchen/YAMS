use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::{domain::ClientId, use_cases};

use crate::{Animal, errors::InternalServerError};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AnimalCreation {
    pub name: String,
    pub birthdate: NaiveDate,
    pub animal_species: String,
    pub description: String,
    pub client_id: Uuid,
}

impl From<AnimalCreation> for use_cases::animals::CreateAnimal {
    fn from(value: AnimalCreation) -> Self {
        Self {
            name: value.name,
            birthdate: value.birthdate,
            animal_species: value.animal_species,
            description: value.description,
            client_id: ClientId(value.client_id),
        }
    }
}
