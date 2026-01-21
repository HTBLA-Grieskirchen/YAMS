use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub id: Option<Uuid>,
    pub country: String,
    pub postal_code: String,
    pub city: String,
    pub street: String,
    pub street_number: String,
    pub extra: String,
}
