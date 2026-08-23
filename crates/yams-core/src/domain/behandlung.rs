use uuid::Uuid;

use crate::domain::Preis;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BehandlungId(pub Uuid);

/// Aggregat
#[derive(Debug, Clone)]
pub struct Behandlung {
    pub id: BehandlungId,
    pub name: String,
    pub beschreibung: String,
    pub standardpreis: Preis,
}

pub struct NeueBehandlung {
    pub name: String,
    pub beschreibung: String,
    pub standardpreis: Preis,
}
