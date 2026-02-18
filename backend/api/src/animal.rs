use chrono::NaiveDate;
use uuid::Uuid;
use yams_core::domain;

api_oai! {
    rename_all = "camelCase";
    api_oai_derive_object!(api_serde! {
        rename_all = "camelCase";
        #[derive(Debug, Clone)]
        pub struct Animal {
            pub id: Uuid,
            pub name: String,
            pub species: String,
            pub birthdate: NaiveDate,
            pub description: String,
        }
    })
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
