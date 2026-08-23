use std::convert::TryFrom;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Ungültige E-Mail-Adresse: {0}")]
pub struct EmailAdresseValidierungsfehler(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailAdresse(String);

impl EmailAdresse {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, EmailAdresseValidierungsfehler> {
        let s_ref = s.as_ref();
        if s_ref.contains('@') && s_ref.contains('.') {
            Ok(Self(s_ref.to_string()))
        } else {
            Err(EmailAdresseValidierungsfehler(s_ref.to_string()))
        }
    }
}

impl TryFrom<String> for EmailAdresse {
    type Error = EmailAdresseValidierungsfehler;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        EmailAdresse::new(s)
    }
}

impl TryFrom<&str> for EmailAdresse {
    type Error = EmailAdresseValidierungsfehler;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        EmailAdresse::new(s)
    }
}

impl AsRef<str> for EmailAdresse {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Error)]
#[error("Ungültige Mobilnummer: {0}")]
pub struct MobilnummerValidierungsfehler(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mobilnummer(String);

impl Mobilnummer {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, MobilnummerValidierungsfehler> {
        let s_ref = s.as_ref();
        let valid = {
            let body = if s_ref.starts_with('+') {
                &s_ref[1..]
            } else {
                s_ref
            };
            body.chars().all(|c| c.is_ascii_digit()) && body.len() >= 7 && body.len() <= 15
        };
        if valid {
            Ok(Self(s_ref.to_string()))
        } else {
            Err(MobilnummerValidierungsfehler(s_ref.to_string()))
        }
    }
}

impl TryFrom<String> for Mobilnummer {
    type Error = MobilnummerValidierungsfehler;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Mobilnummer::new(s)
    }
}

impl TryFrom<&str> for Mobilnummer {
    type Error = MobilnummerValidierungsfehler;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Mobilnummer::new(s)
    }
}

impl AsRef<str> for Mobilnummer {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
