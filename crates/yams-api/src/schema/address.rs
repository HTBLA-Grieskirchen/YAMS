use yams_core::domain;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::NewType))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct CountryCode(pub String);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Address {
    pub postal_code: String,
    pub city: String,
    pub street_and_number: String,
    pub country_code: CountryCode,
}

impl From<Address> for domain::Address {
    fn from(value: Address) -> Self {
        Self {
            postal_code: value.postal_code,
            city: value.city,
            street_and_number: value.street_and_number,
            country_code: value.country_code.0,
        }
    }
}

impl From<domain::Address> for Address {
    fn from(value: domain::Address) -> Self {
        Self {
            postal_code: value.postal_code,
            city: value.city,
            street_and_number: value.street_and_number,
            country_code: CountryCode(value.country_code),
        }
    }
}
