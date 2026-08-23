use thiserror::Error;

#[derive(Debug, Error)]
#[error("Ungültiger Ländercode: {0}")]
pub struct LaendercodeValidierungsfehler(String);

/// ISO 3166-1 alpha-2 (z. B. `DE`, `AT`, `CH`)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ländercode {
    AT,
    DE,
    CH,
}

impl Ländercode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AT => "AT",
            Self::DE => "DE",
            Self::CH => "CH",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, LaendercodeValidierungsfehler> {
        match s {
            "AT" => Ok(Self::AT),
            "DE" => Ok(Self::DE),
            "CH" => Ok(Self::CH),
            _ => Err(LaendercodeValidierungsfehler(s.to_string())),
        }
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
