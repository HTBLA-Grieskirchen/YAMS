use yams_core::domain;

/// E-Mail-Adresse
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::NewType))]
#[cfg_attr(feature = "openapi", oai(example))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct EmailAdresse(String);

#[cfg(feature = "openapi")]
impl poem_openapi::types::Example for EmailAdresse {
    fn example() -> Self {
        Self("test@example.com".to_string())
    }
}

impl From<domain::EmailAdresse> for EmailAdresse {
    fn from(email: domain::EmailAdresse) -> Self {
        Self(email.as_ref().to_string())
    }
}

impl TryFrom<EmailAdresse> for domain::EmailAdresse {
    type Error = domain::kontakt::EmailAdresseValidierungsfehler;
    fn try_from(email: EmailAdresse) -> Result<Self, Self::Error> {
        domain::EmailAdresse::new(email.0)
    }
}

/// Mobilnummer
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::NewType))]
#[cfg_attr(feature = "openapi", oai(example))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Mobilnummer(String);

#[cfg(feature = "openapi")]
impl poem_openapi::types::Example for Mobilnummer {
    fn example() -> Self {
        Self("+43 699 12345678".to_string())
    }
}

impl From<domain::Mobilnummer> for Mobilnummer {
    fn from(mobilnummer: domain::Mobilnummer) -> Self {
        Self(mobilnummer.as_ref().to_string())
    }
}

impl TryFrom<Mobilnummer> for domain::Mobilnummer {
    type Error = domain::kontakt::MobilnummerValidierungsfehler;
    fn try_from(mobilnummer: Mobilnummer) -> Result<Self, Self::Error> {
        domain::Mobilnummer::new(mobilnummer.0)
    }
}
