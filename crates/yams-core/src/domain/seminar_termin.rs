use chrono::{DateTime, Utc};
use error_stack::Report;
use rustc_hash::FxHashMap;
use uuid::Uuid;

use crate::{
    ResultReport,
    domain::{
        Adresse, KlientId, LeistungId, Ratio, Seminar, SeminarId, Zeitraum,
        leistung::NeueLeistung,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeminarTerminId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SeminarBuchungId(pub Uuid);

#[derive(Debug, thiserror::Error)]
pub enum SeminarTerminFehler {
    #[error("maximale teilnehmerzahl erreicht")]
    KapazitätErreicht,
    #[error("klient ist bereits für diesen termin gebucht")]
    KlientBereitsGebucht,
    #[error("buchung nicht gefunden")]
    BuchungNichtGefunden,
    #[error("buchung ist bereits storniert")]
    BuchungBereitsStorniert,
    #[error("neue kapazität liegt unter der anzahl bestätigter buchungen")]
    KapazitätUnterBestätigten,
    #[error("leistungen-mapping deckt die bestätigten buchungen nicht genau ab")]
    LeistungenUnvollständig,
    #[error("termin ist nicht geplant")]
    NichtGeplant,
}

const CONSTRUCTING: &str = "while constructing seminar-termin";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeminarOrt {
    ort_name: Option<String>,
    adresse: Option<Adresse>,
}

impl SeminarOrt {
    pub fn neu(ort_name: Option<String>, adresse: Option<Adresse>) -> Self {
        Self { ort_name, adresse }
    }

    pub fn ort_name(&self) -> Option<&str> {
        self.ort_name.as_deref()
    }

    pub fn adresse(&self) -> Option<&Adresse> {
        self.adresse.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Geplant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abgehalten {
    abgehalten_am: DateTime<Utc>,
    leistungen: FxHashMap<SeminarBuchungId, LeistungId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Abgesagt {
    abgesagt_am: DateTime<Utc>,
    grund: String,
}

#[derive(Debug, Clone)]
pub struct SeminarBuchung {
    id: SeminarBuchungId,
    klient_id: KlientId,
    rabatt: Ratio,
    status: SeminarBuchungStatus,
}

#[derive(Debug, Clone)]
pub enum SeminarBuchungStatus {
    Bestätigt,
    Storniert { storniert_am: DateTime<Utc> },
}

impl SeminarBuchung {
    pub fn neu(id: SeminarBuchungId, klient_id: KlientId, rabatt: Ratio) -> Self {
        Self {
            id,
            klient_id,
            rabatt,
            status: SeminarBuchungStatus::Bestätigt,
        }
    }

    pub fn from_parts(
        id: SeminarBuchungId,
        klient_id: KlientId,
        rabatt: Ratio,
        storniert_am: Option<DateTime<Utc>>,
    ) -> Self {
        let status = match storniert_am {
            Some(storniert_am) => SeminarBuchungStatus::Storniert { storniert_am },
            None => SeminarBuchungStatus::Bestätigt,
        };
        Self {
            id,
            klient_id,
            rabatt,
            status,
        }
    }

    pub fn id(&self) -> &SeminarBuchungId {
        &self.id
    }

    pub fn klient_id(&self) -> &KlientId {
        &self.klient_id
    }

    pub fn rabatt(&self) -> &Ratio {
        &self.rabatt
    }

    pub fn status(&self) -> &SeminarBuchungStatus {
        &self.status
    }

    pub fn ist_bestätigt(&self) -> bool {
        matches!(self.status, SeminarBuchungStatus::Bestätigt)
    }

    pub fn storniert_am(&self) -> Option<DateTime<Utc>> {
        match self.status {
            SeminarBuchungStatus::Storniert { storniert_am } => Some(storniert_am),
            SeminarBuchungStatus::Bestätigt => None,
        }
    }

    fn stornieren(&mut self, storniert_am: DateTime<Utc>) -> ResultReport<(), SeminarTerminFehler> {
        if !self.ist_bestätigt() {
            return Err(Report::new(SeminarTerminFehler::BuchungBereitsStorniert));
        }
        self.status = SeminarBuchungStatus::Storniert { storniert_am };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SeminarTerminIn<S> {
    id: SeminarTerminId,
    seminar_id: SeminarId,
    zeitraum: Zeitraum,
    ort: SeminarOrt,
    max_teilnehmer: Option<u32>,
    buchungen: Vec<SeminarBuchung>,
    state: S,
}

pub type SeminarTerminGeplant = SeminarTerminIn<Geplant>;
pub type SeminarTerminAbgehalten = SeminarTerminIn<Abgehalten>;
pub type SeminarTerminAbgesagt = SeminarTerminIn<Abgesagt>;

#[derive(Debug, Clone)]
pub enum SeminarTermin {
    Geplant(SeminarTerminGeplant),
    Abgehalten(SeminarTerminAbgehalten),
    Abgesagt(SeminarTerminAbgesagt),
}

impl From<SeminarTerminGeplant> for SeminarTermin {
    fn from(value: SeminarTerminGeplant) -> Self {
        Self::Geplant(value)
    }
}

impl From<SeminarTerminAbgehalten> for SeminarTermin {
    fn from(value: SeminarTerminAbgehalten) -> Self {
        Self::Abgehalten(value)
    }
}

impl From<SeminarTerminAbgesagt> for SeminarTermin {
    fn from(value: SeminarTerminAbgesagt) -> Self {
        Self::Abgesagt(value)
    }
}

impl<S> SeminarTerminIn<S> {
    pub fn id(&self) -> &SeminarTerminId {
        &self.id
    }

    pub fn seminar_id(&self) -> &SeminarId {
        &self.seminar_id
    }

    pub fn zeitraum(&self) -> &Zeitraum {
        &self.zeitraum
    }

    pub fn ort(&self) -> &SeminarOrt {
        &self.ort
    }

    pub fn max_teilnehmer(&self) -> Option<u32> {
        self.max_teilnehmer
    }

    pub fn buchungen(&self) -> &[SeminarBuchung] {
        &self.buchungen
    }

    pub fn bestätigte_buchungen(&self) -> impl Iterator<Item = &SeminarBuchung> {
        self.buchungen.iter().filter(|buchung| buchung.ist_bestätigt())
    }

    fn bestätigte_anzahl(&self) -> u32 {
        self.buchungen
            .iter()
            .filter(|buchung| buchung.ist_bestätigt())
            .count() as u32
    }
}

impl SeminarTerminGeplant {
    pub fn neu(
        id: SeminarTerminId,
        neu: NeuerSeminarTermin,
    ) -> ResultReport<Self, crate::domain::zeitraum::ZeitraumFehler> {
        Ok(Self {
            id,
            seminar_id: neu.seminar_id,
            zeitraum: neu.zeitraum,
            ort: neu.ort,
            max_teilnehmer: neu.max_teilnehmer,
            buchungen: Vec::new(),
            state: Geplant,
        })
    }

    pub fn buchung_anlegen(
        &mut self,
        klient_id: KlientId,
        rabatt: Ratio,
    ) -> ResultReport<SeminarBuchungId, SeminarTerminFehler> {
        if self
            .bestätigte_buchungen()
            .any(|buchung| buchung.klient_id() == &klient_id)
        {
            return Err(Report::new(SeminarTerminFehler::KlientBereitsGebucht));
        }
        if let Some(max) = self.max_teilnehmer
            && self.bestätigte_anzahl() >= max
        {
            return Err(Report::new(SeminarTerminFehler::KapazitätErreicht));
        }

        let id = SeminarBuchungId(Uuid::new_v4());
        self.buchungen
            .push(SeminarBuchung::neu(id.clone(), klient_id, rabatt));
        Ok(id)
    }

    pub fn buchung_stornieren(
        &mut self,
        buchung_id: &SeminarBuchungId,
        storniert_am: DateTime<Utc>,
    ) -> ResultReport<(), SeminarTerminFehler> {
        let buchung = self
            .buchungen
            .iter_mut()
            .find(|buchung| buchung.id() == buchung_id)
            .ok_or_else(|| Report::new(SeminarTerminFehler::BuchungNichtGefunden))?;
        buchung.stornieren(storniert_am)
    }

    // TODO: Benachrichtigung an Teilnehmer
    pub fn aktualisieren(
        &mut self,
        ort: SeminarOrt,
        zeitraum: Zeitraum,
        max_teilnehmer: Option<u32>,
    ) -> ResultReport<(), SeminarTerminFehler> {
        if let Some(max) = max_teilnehmer
            && self.bestätigte_anzahl() > max
        {
            return Err(Report::new(SeminarTerminFehler::KapazitätUnterBestätigten));
        }
        self.ort = ort;
        self.zeitraum = zeitraum;
        self.max_teilnehmer = max_teilnehmer;
        Ok(())
    }

    pub fn absagen(self, grund: impl Into<String>, abgesagt_am: DateTime<Utc>) -> SeminarTerminAbgesagt {
        SeminarTerminAbgesagt {
            id: self.id,
            seminar_id: self.seminar_id,
            zeitraum: self.zeitraum,
            ort: self.ort,
            max_teilnehmer: self.max_teilnehmer,
            buchungen: self.buchungen,
            state: Abgesagt {
                abgesagt_am,
                grund: grund.into(),
            },
        }
    }

    pub fn teilnahmeleistungen(&self, seminar: &Seminar) -> Vec<(SeminarBuchungId, NeueLeistung)> {
        self.bestätigte_buchungen()
            .map(|buchung| {
                let gebühr = seminar.teilnahmegebühr_nach_rabatt(buchung.rabatt());
                let beschreibung = format!("{} — {}", seminar.titel(), self.zeitraum);
                let leistung = NeueLeistung::neu(
                    buchung.klient_id().clone(),
                    None,
                    beschreibung,
                    self.zeitraum.ende().date_naive(),
                    crate::domain::LeistungQuelle::Seminar {
                        termin_id: self.id.clone(),
                        buchung_id: buchung.id().clone(),
                        teilnahmegebühr_basis: seminar.teilnahmegebühr_basis().clone(),
                        rabatt: buchung.rabatt().clone(),
                        teilnahmegebühr: gebühr,
                        mwst: seminar.mwst().clone(),
                    },
                );
                (buchung.id().clone(), leistung)
            })
            .collect()
    }

    pub fn als_abgehalten(
        self,
        abgehalten_am: DateTime<Utc>,
        leistungen: FxHashMap<SeminarBuchungId, LeistungId>,
    ) -> ResultReport<SeminarTerminAbgehalten, SeminarTerminFehler> {
        let bestätigt: Vec<SeminarBuchungId> = self
            .bestätigte_buchungen()
            .map(|buchung| buchung.id().clone())
            .collect();

        if leistungen.len() != bestätigt.len()
            || !bestätigt.iter().all(|id| leistungen.contains_key(id))
            || !leistungen.keys().all(|id| bestätigt.contains(id))
        {
            return Err(
                Report::new(SeminarTerminFehler::LeistungenUnvollständig).attach(CONSTRUCTING)
            );
        }

        Ok(SeminarTerminAbgehalten {
            id: self.id,
            seminar_id: self.seminar_id,
            zeitraum: self.zeitraum,
            ort: self.ort,
            max_teilnehmer: self.max_teilnehmer,
            buchungen: self.buchungen,
            state: Abgehalten {
                abgehalten_am,
                leistungen,
            },
        })
    }
}

impl SeminarTerminAbgehalten {
    pub fn abgehalten_am(&self) -> DateTime<Utc> {
        self.state.abgehalten_am
    }

    pub fn leistungen(&self) -> &FxHashMap<SeminarBuchungId, LeistungId> {
        &self.state.leistungen
    }

    pub fn leistung_fuer_buchung(&self, id: &SeminarBuchungId) -> Option<&LeistungId> {
        self.state.leistungen.get(id)
    }
}

impl SeminarTerminAbgesagt {
    pub fn abgesagt_am(&self) -> DateTime<Utc> {
        self.state.abgesagt_am
    }

    pub fn grund(&self) -> &str {
        &self.state.grund
    }
}

impl SeminarTermin {
    pub fn from_parts(
        id: SeminarTerminId,
        seminar_id: SeminarId,
        zeitraum: Zeitraum,
        ort: SeminarOrt,
        max_teilnehmer: Option<u32>,
        buchungen: Vec<SeminarBuchung>,
        zustand: SeminarTerminZustandTeile,
    ) -> Self {
        let geplant = SeminarTerminGeplant {
            id,
            seminar_id,
            zeitraum,
            ort,
            max_teilnehmer,
            buchungen,
            state: Geplant,
        };
        match zustand {
            SeminarTerminZustandTeile::Geplant => Self::Geplant(geplant),
            SeminarTerminZustandTeile::Abgehalten {
                abgehalten_am,
                leistungen,
            } => Self::Abgehalten(SeminarTerminAbgehalten {
                id: geplant.id,
                seminar_id: geplant.seminar_id,
                zeitraum: geplant.zeitraum,
                ort: geplant.ort,
                max_teilnehmer: geplant.max_teilnehmer,
                buchungen: geplant.buchungen,
                state: Abgehalten {
                    abgehalten_am,
                    leistungen,
                },
            }),
            SeminarTerminZustandTeile::Abgesagt {
                abgesagt_am,
                grund,
            } => Self::Abgesagt(geplant.absagen(grund, abgesagt_am)),
        }
    }

    pub fn id(&self) -> &SeminarTerminId {
        match self {
            Self::Geplant(t) => t.id(),
            Self::Abgehalten(t) => t.id(),
            Self::Abgesagt(t) => t.id(),
        }
    }

    pub fn seminar_id(&self) -> &SeminarId {
        match self {
            Self::Geplant(t) => t.seminar_id(),
            Self::Abgehalten(t) => t.seminar_id(),
            Self::Abgesagt(t) => t.seminar_id(),
        }
    }

    pub fn zeitraum(&self) -> &Zeitraum {
        match self {
            Self::Geplant(t) => t.zeitraum(),
            Self::Abgehalten(t) => t.zeitraum(),
            Self::Abgesagt(t) => t.zeitraum(),
        }
    }

    pub fn ort(&self) -> &SeminarOrt {
        match self {
            Self::Geplant(t) => t.ort(),
            Self::Abgehalten(t) => t.ort(),
            Self::Abgesagt(t) => t.ort(),
        }
    }

    pub fn max_teilnehmer(&self) -> Option<u32> {
        match self {
            Self::Geplant(t) => t.max_teilnehmer(),
            Self::Abgehalten(t) => t.max_teilnehmer(),
            Self::Abgesagt(t) => t.max_teilnehmer(),
        }
    }

    pub fn buchungen(&self) -> &[SeminarBuchung] {
        match self {
            Self::Geplant(t) => t.buchungen(),
            Self::Abgehalten(t) => t.buchungen(),
            Self::Abgesagt(t) => t.buchungen(),
        }
    }

    pub fn as_geplant_mut(&mut self) -> Result<&mut SeminarTerminGeplant, SeminarTerminFehler> {
        match self {
            Self::Geplant(termin) => Ok(termin),
            _ => Err(SeminarTerminFehler::NichtGeplant),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SeminarTerminZustandTeile {
    Geplant,
    Abgehalten {
        abgehalten_am: DateTime<Utc>,
        leistungen: FxHashMap<SeminarBuchungId, LeistungId>,
    },
    Abgesagt {
        abgesagt_am: DateTime<Utc>,
        grund: String,
    },
}

#[derive(Debug)]
pub struct NeuerSeminarTermin {
    seminar_id: SeminarId,
    zeitraum: Zeitraum,
    ort: SeminarOrt,
    max_teilnehmer: Option<u32>,
}

impl NeuerSeminarTermin {
    pub fn neu(
        seminar_id: SeminarId,
        zeitraum: Zeitraum,
        ort: SeminarOrt,
        max_teilnehmer: Option<u32>,
    ) -> Self {
        Self {
            seminar_id,
            zeitraum,
            ort,
            max_teilnehmer,
        }
    }

    pub fn seminar_id(&self) -> &SeminarId {
        &self.seminar_id
    }

    pub fn zeitraum(&self) -> &Zeitraum {
        &self.zeitraum
    }

    pub fn ort(&self) -> &SeminarOrt {
        &self.ort
    }

    pub fn max_teilnehmer(&self) -> Option<u32> {
        self.max_teilnehmer
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use rust_decimal::Decimal;

    use super::*;
    use crate::domain::seminar::NeuesSeminar;
    use crate::domain::Preis;

    fn utc(h: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 8, 25, h, 0, 0).unwrap()
    }

    fn zeitraum() -> Zeitraum {
        Zeitraum::neu(utc(10), utc(16)).unwrap()
    }

    fn seminar() -> Seminar {
        Seminar::neu(
            SeminarId(Uuid::new_v4()),
            NeuesSeminar::neu(
                "Hufseminar",
                "Intro",
                Preis::new(Decimal::new(100, 0)).unwrap(),
                Ratio::new(Decimal::new(20, 2)).unwrap(),
                None,
            ),
        )
        .unwrap()
    }

    fn geplant(max: Option<u32>) -> SeminarTerminGeplant {
        SeminarTerminGeplant::neu(
            SeminarTerminId(Uuid::new_v4()),
            NeuerSeminarTermin::neu(
                SeminarId(Uuid::new_v4()),
                zeitraum(),
                SeminarOrt::neu(Some("Hof".into()), None),
                max,
            ),
        )
        .unwrap()
    }

    fn klient() -> KlientId {
        KlientId(Uuid::new_v4())
    }

    fn rabatt_20() -> Ratio {
        Ratio::new(Decimal::new(20, 2)).unwrap()
    }

    #[test]
    fn buchung_anlegen_enforces_capacity() {
        let mut termin = geplant(Some(1));
        termin
            .buchung_anlegen(klient(), Ratio::zero())
            .unwrap();
        let err = termin
            .buchung_anlegen(klient(), Ratio::zero())
            .unwrap_err();
        assert!(matches!(
            err.current_context(),
            SeminarTerminFehler::KapazitätErreicht
        ));
    }

    #[test]
    fn buchung_anlegen_rejects_duplicate_klient() {
        let mut termin = geplant(None);
        let klient = klient();
        termin.buchung_anlegen(klient.clone(), Ratio::zero()).unwrap();
        let err = termin.buchung_anlegen(klient, Ratio::zero()).unwrap_err();
        assert!(matches!(
            err.current_context(),
            SeminarTerminFehler::KlientBereitsGebucht
        ));
    }

    #[test]
    fn storno_frees_slot_and_allows_rebook() {
        let mut termin = geplant(Some(1));
        let klient = klient();
        let buchung_id = termin
            .buchung_anlegen(klient.clone(), Ratio::zero())
            .unwrap();
        termin.buchung_stornieren(&buchung_id, utc(9)).unwrap();
        termin.buchung_anlegen(klient, Ratio::zero()).unwrap();
        assert_eq!(termin.bestätigte_anzahl(), 1);
        assert_eq!(termin.buchungen().len(), 2);
    }

    #[test]
    fn storno_rejects_already_storniert() {
        let mut termin = geplant(None);
        let buchung_id = termin.buchung_anlegen(klient(), Ratio::zero()).unwrap();
        termin.buchung_stornieren(&buchung_id, utc(9)).unwrap();
        let err = termin.buchung_stornieren(&buchung_id, utc(9)).unwrap_err();
        assert!(matches!(
            err.current_context(),
            SeminarTerminFehler::BuchungBereitsStorniert
        ));
    }

    #[test]
    fn aktualisieren_rejects_max_below_confirmed() {
        let mut termin = geplant(Some(2));
        termin.buchung_anlegen(klient(), Ratio::zero()).unwrap();
        termin.buchung_anlegen(klient(), Ratio::zero()).unwrap();
        let err = termin
            .aktualisieren(
                SeminarOrt::neu(None, None),
                zeitraum(),
                Some(1),
            )
            .unwrap_err();
        assert!(matches!(
            err.current_context(),
            SeminarTerminFehler::KapazitätUnterBestätigten
        ));
    }

    #[test]
    fn als_abgehalten_maps_confirmed_only() {
        let mut termin = geplant(None);
        let bestätigt = termin
            .buchung_anlegen(klient(), rabatt_20())
            .unwrap();
        let storniert = termin.buchung_anlegen(klient(), Ratio::zero()).unwrap();
        termin.buchung_stornieren(&storniert, utc(9)).unwrap();

        let leistung_id = LeistungId(Uuid::new_v4());
        let mut mapping = FxHashMap::default();
        mapping.insert(bestätigt.clone(), leistung_id.clone());

        let abgehalten = termin.als_abgehalten(utc(16), mapping).unwrap();
        assert_eq!(
            abgehalten.leistung_fuer_buchung(&bestätigt),
            Some(&leistung_id)
        );
        assert!(abgehalten.leistung_fuer_buchung(&storniert).is_none());
    }

    #[test]
    fn als_abgehalten_rejects_incomplete_mapping() {
        let mut termin = geplant(None);
        termin.buchung_anlegen(klient(), Ratio::zero()).unwrap();
        let err = termin
            .als_abgehalten(utc(16), FxHashMap::default())
            .unwrap_err();
        assert!(matches!(
            err.current_context(),
            SeminarTerminFehler::LeistungenUnvollständig
        ));
    }

    #[test]
    fn absagen_archives_buchungen() {
        let mut termin = geplant(None);
        termin.buchung_anlegen(klient(), Ratio::zero()).unwrap();
        let abgesagt = termin.absagen("zu wenig tn", utc(8));
        assert_eq!(abgesagt.grund(), "zu wenig tn");
        assert_eq!(abgesagt.buchungen().len(), 1);
    }

    #[test]
    fn teilnahmeleistungen_applies_rabatt_snapshot() {
        let seminar = seminar();
        let mut termin = geplant(None);
        termin.buchung_anlegen(klient(), rabatt_20()).unwrap();
        let leistungen = termin.teilnahmeleistungen(&seminar);
        assert_eq!(leistungen.len(), 1);
        match leistungen[0].1.quelle() {
            crate::domain::LeistungQuelle::Seminar {
                teilnahmegebühr,
                rabatt,
                ..
            } => {
                assert_eq!(teilnahmegebühr.value(), Decimal::new(80, 0));
                assert_eq!(rabatt.value(), Decimal::new(20, 2));
            }
            other => panic!("expected seminar quelle, got {other:?}"),
        }
    }
}
