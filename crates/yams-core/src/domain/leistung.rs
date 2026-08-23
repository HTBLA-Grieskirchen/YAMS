use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{
    BehandlungId, HaustierId, KlientId, Preis, ProduktId, RechnungId,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeistungId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeistungStatus {
    Offen,
    Abgerechnet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeistungQuelle {
    Produkt(ProduktId),
    Behandlung(BehandlungId),
    Manuell,
}

/// Aggregat
#[derive(Debug, Clone)]
pub struct Leistung {
    pub id: LeistungId,
    pub klient_id: KlientId,
    pub haustier_id: Option<HaustierId>,
    pub beschreibung: String,
    pub betrag: Preis,
    pub leistungsdatum: NaiveDate,
    pub status: LeistungStatus,
    pub quelle: LeistungQuelle,
    pub rechnung_id: Option<RechnungId>,
}

pub struct NeueLeistung {
    pub klient_id: KlientId,
    pub haustier_id: Option<HaustierId>,
    pub beschreibung: String,
    pub betrag: Preis,
    pub leistungsdatum: NaiveDate,
    pub quelle: LeistungQuelle,
}

#[derive(Debug, thiserror::Error)]
pub enum LeistungFehler {
    #[error("leistung ist bereits abgerechnet")]
    BereitsAbgerechnet,
}

impl Leistung {
    pub fn mark_abgerechnet(&mut self, rechnung_id: RechnungId) -> Result<(), LeistungFehler> {
        if self.status != LeistungStatus::Offen {
            return Err(LeistungFehler::BereitsAbgerechnet);
        }
        self.status = LeistungStatus::Abgerechnet;
        self.rechnung_id = Some(rechnung_id);
        Ok(())
    }
}
