use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::KlientId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HaustierId(pub Uuid);

/// Aggregat
#[derive(Debug, Clone)]
pub struct Haustier {
    pub id: HaustierId,
    pub klient_id: KlientId,
    pub name: String,
    pub geburtstag: NaiveDate,
    pub tierart: String,
    pub beschreibung: String,
}

pub struct NeuesHaustier {
    pub klient_id: KlientId,
    pub name: String,
    pub geburtstag: NaiveDate,
    pub tierart: String,
    pub beschreibung: String,
}
