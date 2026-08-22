/// Value Object for Address
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub postal_code: String,
    pub city: String,
    pub street_and_number: String, // e.g. "Musterstraße 12", "Musterstraße 12"
    pub country_code: String,      // ISO 3166-1 alpha-2 code, e.g. "DE", "AT", "CH"
}
