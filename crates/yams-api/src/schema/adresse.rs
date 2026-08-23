use yams_core::domain;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::NewType))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Ländercode(pub String);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase", example))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Adresse {
    pub postleitzahl: String,
    pub stadt: String,
    pub strasse_und_hausnummer: String,
    pub ländercode: Ländercode,
}

#[cfg(feature = "openapi")]
impl poem_openapi::types::Example for Ländercode {
    fn example() -> Self {
        Self("AT".to_string())
    }
}

#[cfg(feature = "openapi")]
impl poem_openapi::types::Example for Adresse {
    fn example() -> Self {
        Self {
            postleitzahl: "4040".to_string(),
            stadt: "Linz".to_string(),
            strasse_und_hausnummer: "Landesstraße 1".to_string(),
            ländercode: Ländercode("AT".to_string()),
        }
    }
}

impl From<domain::Adresse> for Adresse {
    fn from(value: domain::Adresse) -> Self {
        Self {
            postleitzahl: value.postleitzahl,
            stadt: value.stadt,
            strasse_und_hausnummer: value.strasse_und_hausnummer,
            ländercode: Ländercode(value.ländercode.as_str().to_string()),
        }
    }
}

impl TryFrom<Adresse> for domain::Adresse {
    type Error = domain::adresse::LaendercodeValidierungsfehler;
    fn try_from(value: Adresse) -> Result<Self, Self::Error> {
        Ok(Self {
            postleitzahl: value.postleitzahl,
            stadt: value.stadt,
            strasse_und_hausnummer: value.strasse_und_hausnummer,
            ländercode: domain::Ländercode::from_str(&value.ländercode.0)?,
        })
    }
}
