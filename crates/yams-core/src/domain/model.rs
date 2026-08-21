use chrono::NaiveDate;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

impl<T: AsRef<str>> From<T> for Email {
    fn from(s: T) -> Self {
        Self(s.as_ref().to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MobileNumber(pub String);

impl<T: AsRef<str>> From<T> for MobileNumber {
    fn from(s: T) -> Self {
        Self(s.as_ref().to_string())
    }
}

/// Value Object for Address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub postal_code: String,
    pub city: String,
    pub street_and_number: String, // e.g. "Musterstraße 12", "Musterstraße 12"
    pub country_code: String,      // ISO 3166-1 alpha-2 code, e.g. "DE", "AT", "CH"
}

#[derive(Debug, Clone)]
pub struct Client {
    pub id: ClientId,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDate,
    pub email: Email,
    pub mobile_number: MobileNumber,
    pub customer_number: i64,
    pub consent: bool,
    pub address: Address,
    pub animal_ids: Vec<AnimalId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AnimalId(pub Uuid);

#[derive(Debug, Clone)]
pub struct Animal {
    pub id: AnimalId,
    pub name: String,
    pub birthdate: NaiveDate,
    pub animal_species: String,
    pub description: String,
}
