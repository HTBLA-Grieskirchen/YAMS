use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{KlientId, Leistung, LeistungId, LeistungStatus, Preis};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RechnungId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RechnungStatus {
    Offen,
    Bezahlt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rechnungsposition {
    pub beschreibung: String,
    pub betrag: Preis,
    pub leistung_id: LeistungId,
}

/// Aggregat
#[derive(Debug, Clone)]
pub struct Rechnung {
    pub id: RechnungId,
    pub rechnungsnummer: i64,
    pub klient_id: KlientId,
    pub rechnungsdatum: NaiveDate,
    pub positionen: Vec<Rechnungsposition>,
    pub gesamtbetrag: Preis,
    pub status: RechnungStatus,
}

#[derive(Debug, thiserror::Error)]
pub enum RechnungFehler {
    #[error("keine leistungen vorhanden")]
    KeineLeistungen,
    #[error("leistung gehört nicht zum klient")]
    KlientUnstimmig,
    #[error("leistung ist nicht offen")]
    LeistungNichtOffen,
}

impl Rechnung {
    pub fn aus_leistungen(
        klient_id: KlientId,
        rechnungsnummer: i64,
        rechnungsdatum: NaiveDate,
        leistungen: Vec<Leistung>,
    ) -> Result<Rechnung, RechnungFehler> {
        if leistungen.is_empty() {
            return Err(RechnungFehler::KeineLeistungen);
        }

        let mut gesamtbetrag = Decimal::ZERO;
        let mut positionen = Vec::with_capacity(leistungen.len());

        for leistung in leistungen {
            if leistung.klient_id != klient_id {
                return Err(RechnungFehler::KlientUnstimmig);
            }
            if leistung.status != LeistungStatus::Offen {
                return Err(RechnungFehler::LeistungNichtOffen);
            }

            gesamtbetrag += leistung.betrag.value();
            positionen.push(Rechnungsposition {
                beschreibung: leistung.beschreibung.clone(),
                betrag: leistung.betrag.clone(),
                leistung_id: leistung.id.clone(),
            });
        }

        let gesamtbetrag = Preis::new(gesamtbetrag)
            .expect("summe nicht-negativer beträge ist nicht-negativ");

        Ok(Rechnung {
            id: RechnungId(Uuid::new_v4()),
            rechnungsnummer,
            klient_id,
            rechnungsdatum,
            positionen,
            gesamtbetrag,
            status: RechnungStatus::Offen,
        })
    }
}
