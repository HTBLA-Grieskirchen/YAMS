use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::Preis;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProduktId(pub Uuid);

/// Aggregate
#[derive(Debug, Clone)]
pub struct Produkt {
    pub id: ProduktId,
    pub name: String,
    pub beschreibung: String,
    pub einzelpreis: Preis,
    pub mwst_prozentsatz: Decimal,
}

pub struct NeuesProdukt {
    pub name: String,
    pub beschreibung: String,
    pub einzelpreis: Preis,
    pub mwst_prozentsatz: Decimal,
}
