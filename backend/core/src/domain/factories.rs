use chrono::NaiveDate;

use crate::domain::{Address, Email, MobileNumber};

pub struct NewAnimal {
    pub name: String,
    pub birthdate: NaiveDate,
    pub animal_species: String,
    pub description: String,
}

pub struct NewClient {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDate,
    pub email: Email,
    pub mobile_number: MobileNumber,
    pub customer_number: i64,
    pub address: Address,
    pub consent: bool,
}
