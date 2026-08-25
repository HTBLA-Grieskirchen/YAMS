use chrono::NaiveDate;
use error_stack::Report;
use uuid::Uuid;

use crate::{
    ResultReport,
    domain::{KlientId, Leistung, LeistungId, LeistungOffen, LeistungQuelle, Menge, Preis, Ratio},
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
    stückzahl: Menge,
    mwst: Ratio,
    leistung_id: LeistungId,
}

impl Rechnungsposition {
    pub fn neu(
        beschreibung: String,
        einzelpreis: Preis,
        stückzahl: Menge,
        mwst: Ratio,
        leistung_id: LeistungId,
    ) -> Self {
        Self {
            beschreibung,
            einzelpreis,
            stückzahl,
            mwst,
            leistung_id,
        }
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn einzelpreis(&self) -> &Preis {
        &self.einzelpreis
    }

    pub fn stückzahl(&self) -> &Menge {
        &self.stückzahl
    }

    pub fn mwst(&self) -> &Ratio {
        &self.mwst
    }

    pub fn leistung_id(&self) -> &LeistungId {
        &self.leistung_id
    }

    pub fn gesamtpreis_netto(&self) -> Preis {
        &self.einzelpreis * &self.stückzahl
    }

    pub fn mwst_betrag(&self) -> Preis {
        &self.gesamtpreis_netto() * &self.mwst
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

impl From<RechnungOffen> for Rechnung {
    fn from(value: RechnungOffen) -> Self {
        Self::Offen(value)
    }
}

impl From<RechnungBezahlt> for Rechnung {
    fn from(value: RechnungBezahlt) -> Self {
        Self::Bezahlt(value)
    }
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
        self.positionen.iter().fold(Preis::zero(), |acc, position| {
            acc + position.gesamtpreis_netto()
        })
    }

    pub fn gesamtbetrag_brutto(&self) -> Preis {
        self.positionen.iter().fold(Preis::zero(), |acc, position| {
            acc + position.gesamtpreis_brutto()
        })
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
        leistungen: &mut [&mut Leistung],
    ) -> ResultReport<Self, RechnungFehler> {
        let rechnung_id = RechnungId(Uuid::new_v4());
        let mut positionen = Vec::new();

        for leistung in leistungen.iter_mut() {
            match &*leistung {
                Leistung::Offen(offen) => {
                    if offen.klient_id() != &klient_id {
                        return Err(Report::new(RechnungFehler::KlientUnstimmig));
                    }

                    positionen.push(position_from_leistung(offen));
                    let abgerechnet = offen.clone().mark_abgerechnet(rechnung_id.clone());
                    **leistung = Leistung::from(abgerechnet);
                }
                Leistung::Abgerechnet(_) => {}
            }
        }

        Self::neu(
            rechnung_id,
            rechnungsnummer,
            klient_id,
            rechnungsdatum,
            positionen,
        )
        .map_err(Report::new)
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
        } => (einzelpreis.clone(), menge.clone()),
        LeistungQuelle::Behandlung { preis, .. } | LeistungQuelle::Manuell { preis, .. } => {
            (preis.clone(), Menge::one())
        }
        LeistungQuelle::Seminar { .. } => (leistung.betrag(), Menge::one()),
    };

    Rechnungsposition::neu(
        leistung.beschreibung().to_string(),
        einzelpreis,
        stückzahl,
        leistung.quelle().mwst().clone(),
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

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;
    use crate::domain::leistung::NeueLeistung;

    fn mwst_19() -> Ratio {
        Ratio::new(Decimal::new(19, 2)).unwrap()
    }

    fn position(beschreibung: &str, einzelpreis: i64, stückzahl: i64) -> Rechnungsposition {
        Rechnungsposition::neu(
            beschreibung.into(),
            Preis::new(Decimal::new(einzelpreis, 0)).unwrap(),
            Menge::new(Decimal::new(stückzahl, 0)).unwrap(),
            mwst_19(),
            LeistungId(Uuid::new_v4()),
        )
    }

    fn leistung_offen(klient_id: KlientId, beschreibung: &str, preis: i64) -> Leistung {
        Leistung::from(
            LeistungOffen::neu(
                LeistungId(Uuid::new_v4()),
                NeueLeistung::neu(
                    klient_id,
                    None,
                    beschreibung,
                    NaiveDate::from_ymd_opt(2026, 8, 23).unwrap(),
                    LeistungQuelle::Manuell {
                        preis: Preis::new(Decimal::new(preis, 0)).unwrap(),
                        mwst: mwst_19(),
                    },
                ),
            )
            .unwrap(),
        )
    }

    #[test]
    fn rechnungsposition_berechnet_mwst_korrekt() {
        let position = position("Untersuchung", 100, 1);

        assert_eq!(position.gesamtpreis_netto().value(), Decimal::new(100, 0));
        assert_eq!(position.mwst_betrag().value(), Decimal::new(19, 0));
        assert_eq!(position.gesamtpreis_brutto().value(), Decimal::new(119, 0));
    }

    #[test]
    fn rechnungsposition_mit_stückzahl_multipliziert_netto() {
        let position = position("Futter", 25, 2);

        assert_eq!(position.gesamtpreis_netto().value(), Decimal::new(50, 0));
        assert_eq!(position.mwst_betrag().value(), Decimal::new(95, 1));
        assert_eq!(position.gesamtpreis_brutto().value(), Decimal::new(595, 1));
    }

    #[test]
    fn rechnungsposition_zero_mwst_brutto_equals_netto() {
        let position = Rechnungsposition::neu(
            "Netto".into(),
            Preis::new(Decimal::new(40, 0)).unwrap(),
            Menge::one(),
            Ratio::zero(),
            LeistungId(Uuid::new_v4()),
        );
        assert_eq!(
            position.gesamtpreis_brutto().value(),
            position.gesamtpreis_netto().value()
        );
    }

    #[test]
    fn rechnung_offen_rejects_empty_positionen() {
        let err = RechnungOffen::neu(
            RechnungId(Uuid::new_v4()),
            1,
            KlientId(Uuid::new_v4()),
            NaiveDate::from_ymd_opt(2026, 8, 23).unwrap(),
            vec![],
        )
        .unwrap_err();

        assert!(matches!(err, RechnungFehler::KeineLeistungen));
    }

    #[test]
    fn aus_leistungen_rejects_klient_mismatch() {
        let klient = KlientId(Uuid::new_v4());
        let other = KlientId(Uuid::new_v4());
        let mut fremd = leistung_offen(other, "Fremd", 10);
        let mut leistungen = vec![&mut fremd];

        let err = RechnungOffen::aus_leistungen(
            klient,
            1,
            NaiveDate::from_ymd_opt(2026, 8, 23).unwrap(),
            &mut leistungen,
        )
        .unwrap_err();
        assert!(matches!(
            err.current_context(),
            RechnungFehler::KlientUnstimmig
        ));
    }

    #[test]
    fn aus_leistungen_skips_already_abgerechnet() {
        let klient = KlientId(Uuid::new_v4());
        let mut billed = leistung_offen(klient.clone(), "Alt", 10);
        if let Leistung::Offen(offen) = billed {
            billed = Leistung::from(offen.mark_abgerechnet(RechnungId(Uuid::new_v4())));
        }
        let mut leistungen = vec![&mut billed];

        let err = RechnungOffen::aus_leistungen(
            klient,
            1,
            NaiveDate::from_ymd_opt(2026, 8, 23).unwrap(),
            &mut leistungen,
        )
        .unwrap_err();
        assert!(matches!(
            err.current_context(),
            RechnungFehler::KeineLeistungen
        ));
    }

    #[test]
    fn aus_leistungen_maps_seminar_quelle() {
        let klient = KlientId(Uuid::new_v4());
        let mut seminar = Leistung::from(
            LeistungOffen::neu(
                LeistungId(Uuid::new_v4()),
                NeueLeistung::neu(
                    klient.clone(),
                    None,
                    "Hufseminar",
                    NaiveDate::from_ymd_opt(2026, 8, 25).unwrap(),
                    LeistungQuelle::Seminar {
                        termin_id: crate::domain::SeminarTerminId(Uuid::new_v4()),
                        buchung_id: crate::domain::SeminarBuchungId(Uuid::new_v4()),
                        teilnahmegebühr_basis: Preis::new(Decimal::new(100, 0)).unwrap(),
                        rabatt: Ratio::new(Decimal::new(20, 2)).unwrap(),
                        mwst: mwst_19(),
                    },
                ),
            )
            .unwrap(),
        );
        let mut leistungen = vec![&mut seminar];
        let rechnung = RechnungOffen::aus_leistungen(
            klient,
            1,
            NaiveDate::from_ymd_opt(2026, 8, 25).unwrap(),
            &mut leistungen,
        )
        .unwrap();

        assert_eq!(rechnung.positionen().len(), 1);
        assert_eq!(
            rechnung.positionen()[0].einzelpreis().value(),
            Decimal::new(80, 0)
        );
        assert_eq!(rechnung.positionen()[0].stückzahl().value(), Decimal::ONE);
        assert_eq!(rechnung.gesamtbetrag_brutto().value(), Decimal::new(952, 1));
    }
}
