use chrono::NaiveDate;
use poem_openapi::{ApiResponse, Object, payload::Json};
use uuid::Uuid;
use yams_core::{domain::ClientId, use_cases};

use crate::{Animal, errors::InternalServerError};

#[derive(Object, Serialize, Deserialize, Debug, Clone)]
#[oai(rename_all = "camelCase")]
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

#[derive(ApiResponse)]
pub enum CreateAnimalResponse {
    #[oai(status = 200)]
    Ok(Json<Animal>),
    #[oai(status = 500)]
    InternalError(Json<InternalServerError>),
}
