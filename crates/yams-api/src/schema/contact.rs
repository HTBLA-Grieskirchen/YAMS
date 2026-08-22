use yams_core::domain;

/// Email address
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::NewType))]
#[cfg_attr(feature = "openapi", oai(example))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct EmailAddress(String);

#[cfg(feature = "openapi")]
impl poem_openapi::types::Example for EmailAddress {
    fn example() -> Self {
        Self("test@example.com".to_string())
    }
}

impl From<domain::EmailAddress> for EmailAddress {
    fn from(email: domain::EmailAddress) -> Self {
        Self(email.as_ref().to_string())
    }
}

impl TryFrom<EmailAddress> for domain::EmailAddress {
    type Error = domain::contact::EmailAddressValidationError;
    fn try_from(email: EmailAddress) -> Result<Self, Self::Error> {
        domain::EmailAddress::new(email.0)
    }
}

/// Mobile number
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::NewType))]
#[cfg_attr(feature = "openapi", oai(example))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct MobileNumber(String);

#[cfg(feature = "openapi")]
impl poem_openapi::types::Example for MobileNumber {
    fn example() -> Self {
        Self("+43 699 12345678".to_string())
    }
}

impl From<domain::MobileNumber> for MobileNumber {
    fn from(mobile_number: domain::MobileNumber) -> Self {
        Self(mobile_number.as_ref().to_string())
    }
}

impl TryFrom<MobileNumber> for domain::MobileNumber {
    type Error = domain::contact::MobileNumberValidationError;
    fn try_from(mobile_number: MobileNumber) -> Result<Self, Self::Error> {
        domain::MobileNumber::new(mobile_number.0)
    }
}
