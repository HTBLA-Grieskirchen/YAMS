use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::domain::{BehandlungId, HaustierId, KlientId, Preis, ProduktId, RechnungId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LeistungId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offen;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abgerechnet {
    rechnung_id: RechnungId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeistungQuelle {
    Produkt {
        produkt_id: ProduktId,
        menge: Decimal,
        einzelpreis: Preis,
        mwst_prozentsatz: Decimal,
    },
    Behandlung {
        behandlung_id: BehandlungId,
        preis: Preis,
        mwst_prozentsatz: Decimal,
    },
    Manuell {
        preis: Preis,
        mwst_prozentsatz: Decimal,
    },
}

impl LeistungQuelle {
    pub fn betrag(&self) -> Preis {
        match self {
            Self::Produkt {
                menge, einzelpreis, ..
            } => einzelpreis
                .multiply(*menge)
                .expect("produkt-betrag aus nicht-negativem preis und menge"),
            Self::Behandlung { preis, .. } | Self::Manuell { preis, .. } => preis.clone(),
        }
    }

    pub fn mwst_prozentsatz(&self) -> Decimal {
        match self {
            Self::Produkt {
                mwst_prozentsatz, ..
            } => *mwst_prozentsatz,
            Self::Behandlung {
                mwst_prozentsatz, ..
            } => *mwst_prozentsatz,
            Self::Manuell {
                mwst_prozentsatz, ..
            } => *mwst_prozentsatz,
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
    ) -> Self {
        let offen = LeistungOffen::neu(
            id,
            NeueLeistung {
                klient_id,
                haustier_id,
                beschreibung,
                leistungsdatum,
                quelle,
            },
        );

        match rechnung_id {
            Some(rechnung_id) => Self::from(offen.mark_abgerechnet(rechnung_id)),
            None => Self::from(offen),
        }
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
    pub fn neu(id: LeistungId, leistung: NeueLeistung) -> Self {
        Self {
            id,
            klient_id: leistung.klient_id,
            haustier_id: leistung.haustier_id,
            beschreibung: leistung.beschreibung,
            leistungsdatum: leistung.leistungsdatum,
            quelle: leistung.quelle,
            state: Offen,
        }
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

pub struct NeueLeistung {
    pub klient_id: KlientId,
    pub haustier_id: Option<HaustierId>,
    pub beschreibung: String,
    pub leistungsdatum: NaiveDate,
    pub quelle: LeistungQuelle,
}
