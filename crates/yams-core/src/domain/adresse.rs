use std::convert::TryFrom;

use thiserror::Error;

#[derive(Debug, Error)]
#[error("Ungültiger Ländercode: {0}")]
pub struct LaendercodeValidierungsfehler(String);

/// ISO 3166-1 alpha-2 (z. B. `DE`, `AT`, `CH`)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ländercode(String);

impl Ländercode {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, LaendercodeValidierungsfehler> {
        let s_ref = s.as_ref();
        if s_ref.len() == 2 && s_ref.chars().all(|c| c.is_ascii_uppercase()) {
            Ok(Self(s_ref.to_string()))
        } else {
            Err(LaendercodeValidierungsfehler(s_ref.to_string()))
        }
    }
}

impl TryFrom<String> for Ländercode {
    type Error = LaendercodeValidierungsfehler;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Ländercode::new(s)
    }
}

impl TryFrom<&str> for Ländercode {
    type Error = LaendercodeValidierungsfehler;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ländercode::new(s)
    }
}

impl AsRef<str> for Ländercode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Wertobjekt für Adresse
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Adresse {
    pub postleitzahl: String,
    pub stadt: String,
    pub strasse_und_hausnummer: String,
    pub ländercode: Ländercode,
}
