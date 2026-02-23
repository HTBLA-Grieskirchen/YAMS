use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::domain;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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
