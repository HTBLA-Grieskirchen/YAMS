use thiserror::Error;

#[derive(Debug, Error)]
#[error("Ungültiger Ländercode: {0}")]
pub struct LändercodeValidierungsfehler(String);

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

    pub fn from_str(s: &str) -> Result<Self, LändercodeValidierungsfehler> {
        match s {
            "AT" => Ok(Self::AT),
            "DE" => Ok(Self::DE),
            "CH" => Ok(Self::CH),
            _ => Err(LändercodeValidierungsfehler(s.to_string())),
        }
    }
}

/// Value object for Adresse
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Adresse {
    pub postleitzahl: String,
    pub stadt: String,
    pub straße_und_hausnummer: String,
    pub ländercode: Ländercode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ländercode_from_str_accepts_at_de_ch() {
        assert_eq!(Ländercode::from_str("AT").unwrap(), Ländercode::AT);
        assert_eq!(Ländercode::from_str("DE").unwrap(), Ländercode::DE);
        assert_eq!(Ländercode::from_str("CH").unwrap(), Ländercode::CH);
    }

    #[test]
    fn ländercode_as_str_roundtrip() {
        for code in [Ländercode::AT, Ländercode::DE, Ländercode::CH] {
            assert_eq!(Ländercode::from_str(code.as_str()).unwrap(), code);
        }
    }

    #[test]
    fn ländercode_rejects_unknown_code() {
        assert!(Ländercode::from_str("US").is_err());
        assert!(Ländercode::from_str("de").is_err());
    }
}
