use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::Preis;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BehandlungId(pub Uuid);

/// Aggregate
#[derive(Debug, Clone)]
pub struct Behandlung {
    pub id: BehandlungId,
    pub name: String,
    pub beschreibung: String,
    pub standardpreis: Preis,
    pub mwst_prozentsatz: Decimal,
}

pub struct NeueBehandlung {
    pub name: String,
    pub beschreibung: String,
    pub standardpreis: Preis,
    pub mwst_prozentsatz: Decimal,
}
