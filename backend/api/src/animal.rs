use chrono::{Date, NaiveDate};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use yams_core::domain;

#[derive(Object, Serialize, Deserialize, Debug, Clone)]
#[oai(rename_all = "camelCase")]
pub struct Animal {
    pub id: Uuid,
    pub name: String,
    pub species: String,
    pub birthdate: NaiveDate,
    pub description: String,
}

pub fn schema_animal_from_domain(animal: domain::Animal) -> Animal {
    Animal {
        id: animal.id.0,
        name: animal.name,
        species: animal.animal_species,
        birthdate: animal.birthdate,
        description: animal.description,
    }
}
