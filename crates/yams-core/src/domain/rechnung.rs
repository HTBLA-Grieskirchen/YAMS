use chrono::NaiveDate;
use error_stack::Report;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    ResultReport,
    domain::{KlientId, LeistungAbgerechnet, LeistungId, LeistungOffen, LeistungQuelle, Preis},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RechnungId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RechnungOffenMarker;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RechnungBezahltMarker;

const DEFAULT_MWST_PROZENTSATZ: Decimal = Decimal::from_parts(19, 0, 0, false, 0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rechnungsposition {
    beschreibung: String,
    einzelpreis: Preis,
    stückzahl: Decimal,
    mwst_prozentsatz: Decimal,
    leistung_id: LeistungId,
}

impl Rechnungsposition {
    pub fn neu(
        beschreibung: String,
        einzelpreis: Preis,
        stückzahl: Decimal,
        mwst_prozentsatz: Decimal,
        leistung_id: LeistungId,
    ) -> Self {
        Self {
            beschreibung,
            einzelpreis,
            stückzahl,
            mwst_prozentsatz,
            leistung_id,
        }
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn einzelpreis(&self) -> &Preis {
        &self.einzelpreis
    }

    pub fn stückzahl(&self) -> Decimal {
        self.stückzahl
    }

    pub fn mwst_prozentsatz(&self) -> Decimal {
        self.mwst_prozentsatz
    }

    pub fn leistung_id(&self) -> &LeistungId {
        &self.leistung_id
    }

    pub fn gesamtpreis_netto(&self) -> Preis {
        self.einzelpreis
            .multiply(self.stückzahl)
            .expect("nettopreis aus nicht-negativem einzelpreis und stückzahl")
    }

    pub fn mwst_betrag(&self) -> Preis {
        let netto = self.gesamtpreis_netto().value();
        let mwst = netto * self.mwst_prozentsatz / Decimal::from(100);
        Preis::new(mwst).expect("mwst-betrag aus nicht-negativem netto")
    }

    pub fn gesamtpreis_brutto(&self) -> Preis {
        self.gesamtpreis_netto()
            .add(&self.mwst_betrag())
            .expect("bruttopreis aus nicht-negativem netto und mwst")
    }
}

#[derive(Debug, Clone)]
pub struct Rechnung<S> {
    id: RechnungId,
    rechnungsnummer: i64,
    klient_id: KlientId,
    rechnungsdatum: NaiveDate,
    positionen: Vec<Rechnungsposition>,
    state: S,
}

pub type RechnungOffen = Rechnung<RechnungOffenMarker>;
pub type RechnungBezahlt = Rechnung<RechnungBezahltMarker>;

#[derive(Debug, Clone)]
pub enum GeladeneRechnung {
    Offen(RechnungOffen),
    Bezahlt(RechnungBezahlt),
}

impl<S> Rechnung<S> {
    pub fn id(&self) -> &RechnungId {
        &self.id
    }

    pub fn rechnungsnummer(&self) -> i64 {
        self.rechnungsnummer
    }

    pub fn klient_id(&self) -> &KlientId {
        &self.klient_id
    }

    pub fn rechnungsdatum(&self) -> NaiveDate {
        self.rechnungsdatum
    }

    pub fn positionen(&self) -> &[Rechnungsposition] {
        &self.positionen
    }

    pub fn gesamtbetrag_netto(&self) -> Preis {
        self.positionen
            .iter()
            .fold(Preis::zero(), |acc, position| {
                acc.add(&position.gesamtpreis_netto())
                    .expect("summe nicht-negativer nettopreise")
            })
    }

    pub fn gesamtbetrag_brutto(&self) -> Preis {
        self.positionen
            .iter()
            .fold(Preis::zero(), |acc, position| {
                acc.add(&position.gesamtpreis_brutto())
                    .expect("summe nicht-negativer bruttopreise")
            })
    }
}

impl RechnungOffen {
    pub fn neu(
        id: RechnungId,
        rechnungsnummer: i64,
        klient_id: KlientId,
        rechnungsdatum: NaiveDate,
        positionen: Vec<Rechnungsposition>,
    ) -> Result<Self, RechnungFehler> {
        if positionen.is_empty() {
            return Err(RechnungFehler::KeineLeistungen);
        }

        Ok(Self {
            id,
            rechnungsnummer,
            klient_id,
            rechnungsdatum,
            positionen,
            state: RechnungOffenMarker,
        })
    }

    pub fn aus_leistungen(
        klient_id: KlientId,
        rechnungsnummer: i64,
        rechnungsdatum: NaiveDate,
        leistungen: &mut Vec<LeistungOffen>,
    ) -> ResultReport<(Self, Vec<LeistungAbgerechnet>), RechnungFehler> {
        if leistungen.is_empty() {
            return Err(Report::new(RechnungFehler::KeineLeistungen));
        }

        let rechnung_id = RechnungId(Uuid::new_v4());
        let mut positionen = Vec::with_capacity(leistungen.len());
        let mut abgerechnet = Vec::with_capacity(leistungen.len());

        for leistung in leistungen.drain(..) {
            if leistung.klient_id() != &klient_id {
                return Err(Report::new(RechnungFehler::KlientUnstimmig));
            }

            positionen.push(position_from_leistung(&leistung));
            abgerechnet.push(leistung.mark_abgerechnet(rechnung_id.clone()));
        }

        let rechnung = Self {
            id: rechnung_id,
            rechnungsnummer,
            klient_id,
            rechnungsdatum,
            positionen,
            state: RechnungOffenMarker,
        };

        Ok((rechnung, abgerechnet))
    }
}

impl GeladeneRechnung {
    pub fn from_parts(
        id: RechnungId,
        rechnungsnummer: i64,
        klient_id: KlientId,
        rechnungsdatum: NaiveDate,
        positionen: Vec<Rechnungsposition>,
        bezahlt: bool,
    ) -> Result<Self, RechnungFehler> {
        if positionen.is_empty() {
            return Err(RechnungFehler::KeineLeistungen);
        }

        if bezahlt {
            Ok(Self::Bezahlt(Rechnung {
                id,
                rechnungsnummer,
                klient_id,
                rechnungsdatum,
                positionen,
                state: RechnungBezahltMarker,
            }))
        } else {
            Ok(Self::Offen(Rechnung {
                id,
                rechnungsnummer,
                klient_id,
                rechnungsdatum,
                positionen,
                state: RechnungOffenMarker,
            }))
        }
    }

    pub fn id(&self) -> &RechnungId {
        match self {
            Self::Offen(r) => r.id(),
            Self::Bezahlt(r) => r.id(),
        }
    }

    pub fn rechnungsnummer(&self) -> i64 {
        match self {
            Self::Offen(r) => r.rechnungsnummer(),
            Self::Bezahlt(r) => r.rechnungsnummer(),
        }
    }

    pub fn klient_id(&self) -> &KlientId {
        match self {
            Self::Offen(r) => r.klient_id(),
            Self::Bezahlt(r) => r.klient_id(),
        }
    }
}

fn position_from_leistung(leistung: &LeistungOffen) -> Rechnungsposition {
    let (einzelpreis, stückzahl) = match leistung.quelle() {
        LeistungQuelle::Produkt {
            einzelpreis, menge, ..
        } => (einzelpreis.clone(), *menge),
        LeistungQuelle::Behandlung { preis, .. } | LeistungQuelle::Manuell { preis } => {
            (preis.clone(), Decimal::ONE)
        }
    };

    Rechnungsposition::neu(
        leistung.beschreibung().to_string(),
        einzelpreis,
        stückzahl,
        DEFAULT_MWST_PROZENTSATZ,
        leistung.id().clone(),
    )
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
