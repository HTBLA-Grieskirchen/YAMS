use chrono::TimeDelta;
use error_stack::Report;
use uuid::Uuid;

use crate::{
    ResultReport,
    domain::{Preis, Ratio},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeminarId(pub Uuid);

#[derive(Debug, thiserror::Error)]
pub enum SeminarFehler {
    #[error("titel darf nicht leer sein")]
    TitelLeer,
}

const CONSTRUCTING: &str = "while constructing seminar";

/// Stammdaten-Vorlage für Seminare.
#[derive(Debug, Clone)]
pub struct Seminar {
    id: SeminarId,
    titel: String,
    beschreibung: String,
    teilnahmegebühr_basis: Preis,
    mwst: Ratio,
    standarddauer: Option<TimeDelta>,
}

impl Seminar {
    pub fn neu(id: SeminarId, neu: NeuesSeminar) -> ResultReport<Self, SeminarFehler> {
        if neu.titel.trim().is_empty() {
            return Err(Report::new(SeminarFehler::TitelLeer).attach(CONSTRUCTING));
        }
        Ok(Self {
            id,
            titel: neu.titel,
            beschreibung: neu.beschreibung,
            teilnahmegebühr_basis: neu.teilnahmegebühr_basis,
            mwst: neu.mwst,
            standarddauer: neu.standarddauer,
        })
    }

    pub fn from_parts(
        id: SeminarId,
        titel: String,
        beschreibung: String,
        teilnahmegebühr_basis: Preis,
        mwst: Ratio,
        standarddauer: Option<TimeDelta>,
    ) -> ResultReport<Self, SeminarFehler> {
        Self::neu(
            id,
            NeuesSeminar::neu(
                titel,
                beschreibung,
                teilnahmegebühr_basis,
                mwst,
                standarddauer,
            ),
        )
    }

    pub fn id(&self) -> &SeminarId {
        &self.id
    }

    pub fn titel(&self) -> &str {
        &self.titel
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn teilnahmegebühr_basis(&self) -> &Preis {
        &self.teilnahmegebühr_basis
    }

    pub fn mwst(&self) -> &Ratio {
        &self.mwst
    }

    pub fn standarddauer(&self) -> Option<TimeDelta> {
        self.standarddauer
    }

    pub fn teilnahmegebühr_nach_rabatt(&self, rabatt: &Ratio) -> Preis {
        self.teilnahmegebühr_basis.nach_rabatt(rabatt)
    }
}

#[derive(Debug)]
pub struct NeuesSeminar {
    titel: String,
    beschreibung: String,
    teilnahmegebühr_basis: Preis,
    mwst: Ratio,
    standarddauer: Option<TimeDelta>,
}

impl NeuesSeminar {
    pub fn neu(
        titel: impl Into<String>,
        beschreibung: impl Into<String>,
        teilnahmegebühr_basis: Preis,
        mwst: Ratio,
        standarddauer: Option<TimeDelta>,
    ) -> Self {
        Self {
            titel: titel.into(),
            beschreibung: beschreibung.into(),
            teilnahmegebühr_basis,
            mwst,
            standarddauer,
        }
    }

    pub fn titel(&self) -> &str {
        &self.titel
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn teilnahmegebühr_basis(&self) -> &Preis {
        &self.teilnahmegebühr_basis
    }

    pub fn mwst(&self) -> &Ratio {
        &self.mwst
    }

    pub fn standarddauer(&self) -> Option<TimeDelta> {
        self.standarddauer
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    fn preis() -> Preis {
        Preis::new(Decimal::new(80, 0)).unwrap()
    }

    fn seminar(titel: &str) -> ResultReport<Seminar, SeminarFehler> {
        Seminar::neu(
            SeminarId(Uuid::new_v4()),
            NeuesSeminar::neu(titel, "Intro", preis(), Ratio::zero(), None),
        )
    }

    #[test]
    fn seminar_rejects_empty_titel() {
        let err = seminar("  ").unwrap_err();
        assert!(matches!(err.current_context(), SeminarFehler::TitelLeer));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }

    #[test]
    fn seminar_accepts_titel_and_rabatt() {
        let seminar = seminar("Hufseminar").unwrap();
        let rabatt = Ratio::new(Decimal::new(20, 2)).unwrap();
        assert_eq!(
            seminar.teilnahmegebühr_nach_rabatt(&rabatt).value(),
            Decimal::new(64, 0)
        );
    }
}
