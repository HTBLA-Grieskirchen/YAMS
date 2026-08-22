use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimalId(pub Uuid);

/// Aggregate
#[derive(Debug, Clone)]
pub struct Animal {
    pub id: AnimalId,
    pub name: String,
    pub birthdate: NaiveDate,
    pub animal_species: String,
    pub description: String,
}

pub struct NewAnimal {
    pub name: String,
    pub birthdate: NaiveDate,
    pub animal_species: String,
    pub description: String,
}
