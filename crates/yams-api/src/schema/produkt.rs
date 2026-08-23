use rust_decimal::Decimal;
use yams_core::domain;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Produkt {
    pub id: uuid::Uuid,
    pub name: String,
    pub beschreibung: String,
    pub einzelpreis: Decimal,
    pub mwst_prozentsatz: Decimal,
}

pub fn schema_produkt_from_domain(produkt: domain::Produkt) -> Produkt {
    Produkt {
        id: produkt.id.0,
        name: produkt.name,
        beschreibung: produkt.beschreibung,
        einzelpreis: produkt.einzelpreis.value(),
        mwst_prozentsatz: produkt.mwst_prozentsatz,
    }
}
