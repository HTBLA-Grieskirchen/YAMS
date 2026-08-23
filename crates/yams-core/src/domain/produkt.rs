use uuid::Uuid;

use crate::domain::Preis;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProduktId(pub Uuid);

/// Aggregat
#[derive(Debug, Clone)]
pub struct Produkt {
    pub id: ProduktId,
    pub name: String,
    pub beschreibung: String,
    pub einzelpreis: Preis,
}

pub struct NeuesProdukt {
    pub name: String,
    pub beschreibung: String,
    pub einzelpreis: Preis,
}
