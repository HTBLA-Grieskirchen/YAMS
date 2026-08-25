use chrono::NaiveDate;
use error_stack::Report;
use uuid::Uuid;

use crate::{
    ResultReport,
    domain::{
        BehandlungId, HaustierId, KlientId, Menge, Preis, ProduktId, Ratio, RechnungId,
        SeminarBuchungId, SeminarTerminId,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeistungId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offen;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abgerechnet {
    rechnung_id: RechnungId,
}

#[derive(Debug, thiserror::Error)]
pub enum LeistungFehler {
    #[error("beschreibung darf nicht leer sein")]
    BeschreibungLeer,
}

const CONSTRUCTING: &str = "while constructing leistung";

#[derive(Debug, Clone)]
pub enum LeistungQuelle {
    Produkt {
        produkt_id: ProduktId,
        menge: Menge,
        einzelpreis: Preis,
        mwst: Ratio,
    },
    Behandlung {
        behandlung_id: BehandlungId,
        preis: Preis,
        mwst: Ratio,
    },
    Manuell {
        preis: Preis,
        mwst: Ratio,
    },
    Seminar {
        termin_id: SeminarTerminId,
        buchung_id: SeminarBuchungId,
        teilnahmegebühr_basis: Preis,
        rabatt: Ratio,
        teilnahmegebühr: Preis,
        mwst: Ratio,
    },
}

impl LeistungQuelle {
    pub fn betrag(&self) -> Preis {
        match self {
            Self::Produkt {
                menge, einzelpreis, ..
            } => einzelpreis * menge,
            Self::Behandlung { preis, .. } | Self::Manuell { preis, .. } => preis.clone(),
            Self::Seminar {
                teilnahmegebühr, ..
            } => teilnahmegebühr.clone(),
        }
    }

    pub fn mwst(&self) -> &Ratio {
        match self {
            Self::Produkt { mwst, .. } => mwst,
            Self::Behandlung { mwst, .. } => mwst,
            Self::Manuell { mwst, .. } => mwst,
            Self::Seminar { mwst, .. } => mwst,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LeistungIn<S> {
    id: LeistungId,
    klient_id: KlientId,
    haustier_id: Option<HaustierId>,
    beschreibung: String,
    leistungsdatum: NaiveDate,
    quelle: LeistungQuelle,
    state: S,
}

pub type LeistungOffen = LeistungIn<Offen>;
pub type LeistungAbgerechnet = LeistungIn<Abgerechnet>;

#[derive(Debug, Clone)]
pub enum Leistung {
    Offen(LeistungOffen),
    Abgerechnet(LeistungAbgerechnet),
}

impl From<LeistungOffen> for Leistung {
    fn from(value: LeistungOffen) -> Self {
        Self::Offen(value)
    }
}

impl From<LeistungAbgerechnet> for Leistung {
    fn from(value: LeistungAbgerechnet) -> Self {
        Self::Abgerechnet(value)
    }
}

impl Leistung {
    pub fn from_parts(
        id: LeistungId,
        klient_id: KlientId,
        haustier_id: Option<HaustierId>,
        beschreibung: String,
        leistungsdatum: NaiveDate,
        quelle: LeistungQuelle,
        rechnung_id: Option<RechnungId>,
    ) -> ResultReport<Self, LeistungFehler> {
        let offen = LeistungOffen::neu(
            id,
            NeueLeistung::neu(klient_id, haustier_id, beschreibung, leistungsdatum, quelle),
        )?;

        Ok(match rechnung_id {
            Some(rechnung_id) => Self::from(offen.mark_abgerechnet(rechnung_id)),
            None => Self::from(offen),
        })
    }

    pub fn id(&self) -> &LeistungId {
        match self {
            Self::Offen(l) => l.id(),
            Self::Abgerechnet(l) => l.id(),
        }
    }

    pub fn klient_id(&self) -> &KlientId {
        match self {
            Self::Offen(l) => l.klient_id(),
            Self::Abgerechnet(l) => l.klient_id(),
        }
    }
}

impl<S> LeistungIn<S> {
    pub fn id(&self) -> &LeistungId {
        &self.id
    }

    pub fn klient_id(&self) -> &KlientId {
        &self.klient_id
    }

    pub fn haustier_id(&self) -> &Option<HaustierId> {
        &self.haustier_id
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn leistungsdatum(&self) -> NaiveDate {
        self.leistungsdatum
    }

    pub fn quelle(&self) -> &LeistungQuelle {
        &self.quelle
    }

    pub fn betrag(&self) -> Preis {
        self.quelle.betrag()
    }
}

impl LeistungOffen {
    pub fn neu(id: LeistungId, leistung: NeueLeistung) -> ResultReport<Self, LeistungFehler> {
        if leistung.beschreibung.trim().is_empty() {
            return Err(Report::new(LeistungFehler::BeschreibungLeer).attach(CONSTRUCTING));
        }
        Ok(Self {
            id,
            klient_id: leistung.klient_id,
            haustier_id: leistung.haustier_id,
            beschreibung: leistung.beschreibung,
            leistungsdatum: leistung.leistungsdatum,
            quelle: leistung.quelle,
            state: Offen,
        })
    }

    pub fn mark_abgerechnet(self, rechnung_id: RechnungId) -> LeistungAbgerechnet {
        LeistungAbgerechnet {
            id: self.id,
            klient_id: self.klient_id,
            haustier_id: self.haustier_id,
            beschreibung: self.beschreibung,
            leistungsdatum: self.leistungsdatum,
            quelle: self.quelle,
            state: Abgerechnet { rechnung_id },
        }
    }
}

impl LeistungAbgerechnet {
    pub fn rechnung_id(&self) -> &RechnungId {
        &self.state.rechnung_id
    }
}

#[derive(Debug)]
pub struct NeueLeistung {
    klient_id: KlientId,
    haustier_id: Option<HaustierId>,
    beschreibung: String,
    leistungsdatum: NaiveDate,
    quelle: LeistungQuelle,
}

impl NeueLeistung {
    pub fn neu(
        klient_id: KlientId,
        haustier_id: Option<HaustierId>,
        beschreibung: impl Into<String>,
        leistungsdatum: NaiveDate,
        quelle: LeistungQuelle,
    ) -> Self {
        Self {
            klient_id,
            haustier_id,
            beschreibung: beschreibung.into(),
            leistungsdatum,
            quelle,
        }
    }

    pub fn klient_id(&self) -> &KlientId {
        &self.klient_id
    }

    pub fn haustier_id(&self) -> Option<&HaustierId> {
        self.haustier_id.as_ref()
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn leistungsdatum(&self) -> NaiveDate {
        self.leistungsdatum
    }

    pub fn quelle(&self) -> &LeistungQuelle {
        &self.quelle
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    #[test]
    fn produkt_betrag_multiplies_menge() {
        let quelle = LeistungQuelle::Produkt {
            produkt_id: ProduktId(Uuid::new_v4()),
            menge: Menge::new(Decimal::new(2, 0)).unwrap(),
            einzelpreis: Preis::new(Decimal::new(25, 0)).unwrap(),
            mwst: Ratio::new(Decimal::new(20, 2)).unwrap(),
        };

        assert_eq!(quelle.betrag().value(), Decimal::new(50, 0));
    }

    #[test]
    fn behandlung_betrag_uses_snapshot_preis() {
        let quelle = LeistungQuelle::Behandlung {
            behandlung_id: BehandlungId(Uuid::new_v4()),
            preis: Preis::new(Decimal::new(50, 0)).unwrap(),
            mwst: Ratio::new(Decimal::new(20, 2)).unwrap(),
        };

        assert_eq!(quelle.betrag().value(), Decimal::new(50, 0));
    }

    #[test]
    fn seminar_betrag_uses_teilnahmegebühr_snapshot() {
        let quelle = LeistungQuelle::Seminar {
            termin_id: crate::domain::SeminarTerminId(Uuid::new_v4()),
            buchung_id: crate::domain::SeminarBuchungId(Uuid::new_v4()),
            teilnahmegebühr_basis: Preis::new(Decimal::new(100, 0)).unwrap(),
            rabatt: Ratio::new(Decimal::new(20, 2)).unwrap(),
            teilnahmegebühr: Preis::new(Decimal::new(80, 0)).unwrap(),
            mwst: Ratio::new(Decimal::new(20, 2)).unwrap(),
        };

        assert_eq!(quelle.betrag().value(), Decimal::new(80, 0));
        assert_eq!(quelle.mwst().value(), Decimal::new(20, 2));
    }

    #[test]
    fn leistung_rejects_empty_beschreibung() {
        let err = LeistungOffen::neu(
            LeistungId(Uuid::new_v4()),
            NeueLeistung::neu(
                KlientId(Uuid::new_v4()),
                None,
                "   ",
                NaiveDate::from_ymd_opt(2026, 8, 23).unwrap(),
                LeistungQuelle::Manuell {
                    preis: Preis::new(Decimal::new(30, 0)).unwrap(),
                    mwst: Ratio::zero(),
                },
            ),
        )
        .unwrap_err();
        assert!(matches!(
            err.current_context(),
            LeistungFehler::BeschreibungLeer
        ));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }

    #[test]
    fn mark_abgerechnet_sets_rechnung_id() {
        let rechnung_id = RechnungId(Uuid::new_v4());
        let offen = LeistungOffen::neu(
            LeistungId(Uuid::new_v4()),
            NeueLeistung::neu(
                KlientId(Uuid::new_v4()),
                None,
                "Untersuchung",
                NaiveDate::from_ymd_opt(2026, 8, 23).unwrap(),
                LeistungQuelle::Manuell {
                    preis: Preis::new(Decimal::new(30, 0)).unwrap(),
                    mwst: Ratio::zero(),
                },
            ),
        )
        .unwrap();
        let abgerechnet = offen.mark_abgerechnet(rechnung_id.clone());
        assert_eq!(abgerechnet.rechnung_id(), &rechnung_id);

        let reconstructed = Leistung::from_parts(
            abgerechnet.id().clone(),
            abgerechnet.klient_id().clone(),
            abgerechnet.haustier_id().clone(),
            abgerechnet.beschreibung().to_string(),
            abgerechnet.leistungsdatum(),
            abgerechnet.quelle().clone(),
            Some(rechnung_id.clone()),
        )
        .unwrap();
        match reconstructed {
            Leistung::Abgerechnet(l) => assert_eq!(l.rechnung_id(), &rechnung_id),
            Leistung::Offen(_) => panic!("expected abgerechnet"),
        }
    }
}
