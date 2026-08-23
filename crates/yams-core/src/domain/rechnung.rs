use chrono::NaiveDate;
use error_stack::Report;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::{
    ResultReport,
    domain::{KlientId, Leistung, LeistungId, LeistungOffen, LeistungQuelle, Preis},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RechnungId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offen;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bezahlt {
    bezahlt_datum: NaiveDate,
}

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
        self.gesamtpreis_netto() + self.mwst_betrag()
    }
}

#[derive(Debug, Clone)]
pub struct RechnungIn<S> {
    id: RechnungId,
    rechnungsnummer: u64,
    klient_id: KlientId,
    rechnungsdatum: NaiveDate,
    positionen: Vec<Rechnungsposition>,
    state: S,
}

pub type RechnungOffen = RechnungIn<Offen>;
pub type RechnungBezahlt = RechnungIn<Bezahlt>;

#[derive(Debug, Clone)]
pub enum Rechnung {
    Offen(RechnungOffen),
    Bezahlt(RechnungBezahlt),
}

impl<S> RechnungIn<S> {
    pub fn id(&self) -> &RechnungId {
        &self.id
    }

    pub fn rechnungsnummer(&self) -> u64 {
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
            .fold(Preis::zero(), |acc, position| acc + position.gesamtpreis_netto())
    }

    pub fn gesamtbetrag_brutto(&self) -> Preis {
        self.positionen
            .iter()
            .fold(Preis::zero(), |acc, position| acc + position.gesamtpreis_brutto())
    }
}

impl RechnungOffen {
    pub fn neu(
        id: RechnungId,
        rechnungsnummer: u64,
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
            state: Offen,
        })
    }

    pub fn aus_leistungen(
        klient_id: KlientId,
        rechnungsnummer: u64,
        rechnungsdatum: NaiveDate,
        leistungen: &mut Vec<Leistung>,
    ) -> ResultReport<Self, RechnungFehler> {
        let mut offene = Vec::new();
        let mut andere = Vec::new();

        for leistung in leistungen.drain(..) {
            match leistung {
                Leistung::Offen(o) => offene.push(o),
                other => andere.push(other),
            }
        }

        if offene.is_empty() {
            *leistungen = andere;
            return Err(Report::new(RechnungFehler::KeineLeistungen));
        }

        let rechnung_id = RechnungId(Uuid::new_v4());
        let mut positionen = Vec::with_capacity(offene.len());

        for leistung in offene {
            if leistung.klient_id() != &klient_id {
                *leistungen = andere;
                return Err(Report::new(RechnungFehler::KlientUnstimmig));
            }

            positionen.push(position_from_leistung(&leistung));
            andere.push(Leistung::Abgerechnet(leistung.mark_abgerechnet(rechnung_id.clone())));
        }

        *leistungen = andere;

        Ok(Self {
            id: rechnung_id,
            rechnungsnummer,
            klient_id,
            rechnungsdatum,
            positionen,
            state: Offen,
        })
    }
}

impl RechnungBezahlt {
    pub fn bezahlt_datum(&self) -> NaiveDate {
        self.state.bezahlt_datum
    }
}

impl Rechnung {
    pub fn from_parts(
        id: RechnungId,
        rechnungsnummer: u64,
        klient_id: KlientId,
        rechnungsdatum: NaiveDate,
        positionen: Vec<Rechnungsposition>,
        bezahlt_datum: Option<NaiveDate>,
    ) -> Result<Self, RechnungFehler> {
        if positionen.is_empty() {
            return Err(RechnungFehler::KeineLeistungen);
        }

        match bezahlt_datum {
            Some(bezahlt_datum) => Ok(Self::Bezahlt(RechnungIn {
                id,
                rechnungsnummer,
                klient_id,
                rechnungsdatum,
                positionen,
                state: Bezahlt { bezahlt_datum },
            })),
            None => Ok(Self::Offen(RechnungIn {
                id,
                rechnungsnummer,
                klient_id,
                rechnungsdatum,
                positionen,
                state: Offen,
            })),
        }
    }

    pub fn id(&self) -> &RechnungId {
        match self {
            Self::Offen(r) => r.id(),
            Self::Bezahlt(r) => r.id(),
        }
    }

    pub fn rechnungsnummer(&self) -> u64 {
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
        LeistungQuelle::Behandlung { preis, .. } | LeistungQuelle::Manuell { preis, .. } => {
            (preis.clone(), Decimal::ONE)
        }
    };

    Rechnungsposition::neu(
        leistung.beschreibung().to_string(),
        einzelpreis,
        stückzahl,
        leistung.quelle().mwst_prozentsatz(),
        leistung.id().clone(),
    )
}

#[derive(Debug, thiserror::Error)]
pub enum RechnungFehler {
    #[error("keine leistungen vorhanden")]
    KeineLeistungen,
    #[error("leistung gehört nicht zum klient")]
    KlientUnstimmig,
}
