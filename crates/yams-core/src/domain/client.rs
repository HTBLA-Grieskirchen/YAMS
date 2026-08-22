use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{Address, AnimalId, EmailAddress, MobileNumber};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientId(pub Uuid);

/// Aggregate
#[derive(Debug, Clone)]
pub struct Client {
    pub id: ClientId,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDate,
    pub email: EmailAddress,
    pub mobile_number: MobileNumber,
    pub customer_number: i64,
    pub consent: bool,
    pub address: Address,
    pub animal_ids: Vec<AnimalId>,
}

pub struct NewClient {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: NaiveDate,
    pub email: EmailAddress,
    pub mobile_number: MobileNumber,
    pub customer_number: i64,
    pub address: Address,
    pub consent: bool,
}
