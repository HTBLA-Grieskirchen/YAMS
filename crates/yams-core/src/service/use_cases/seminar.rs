use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{Report, ResultExt};
use rustc_hash::FxHashMap;
use std::ops::DerefMut;

use crate::{
    application::uow::Versioned,
    domain::{
        KlientId, Preis, Ratio, Seminar, SeminarBuchungId, SeminarId, SeminarOrt, SeminarTermin,
        SeminarTerminGeplant, SeminarTerminId, Zeitraum, seminar::NeuesSeminar,
        seminar_termin::NeuerSeminarTermin,
    },
    service::{
        ExecutionContext, UseCase,
        pdf::{teilnahme_dokument, teilnahme_object_key},
    },
};

#[derive(Clone)]
pub struct SeminarErstellen {
    pub titel: String,
    pub beschreibung: String,
    pub teilnahmegebühr_basis: Preis,
    pub mwst: Ratio,
    pub standarddauer: Option<chrono::TimeDelta>,
}

#[derive(thiserror::Error, Debug)]
pub enum SeminarErstellenFehler {
    #[error("fehler beim anlegen des seminars")]
    Erstellung,
}

#[async_trait]
impl UseCase<Seminar> for SeminarErstellen {
    type Error = Report<SeminarErstellenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Seminar, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;
        uow.seminare()
            .create(NeuesSeminar::neu(
                self.titel,
                self.beschreibung,
                self.teilnahmegebühr_basis,
                self.mwst,
                self.standarddauer,
            ))
            .await
            .map(Versioned::into_data)
            .change_context(SeminarErstellenFehler::Erstellung)
    }
}

#[derive(Clone)]
pub struct SeminarTerminPlanen {
    pub seminar_id: SeminarId,
    pub zeitraum: Zeitraum,
    pub ort: SeminarOrt,
    pub max_teilnehmer: Option<u32>,
}

#[derive(thiserror::Error, Debug)]
pub enum SeminarTerminPlanenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("seminar nicht gefunden")]
    SeminarNichtGefunden,
}

#[async_trait]
impl UseCase<SeminarTerminGeplant> for SeminarTerminPlanen {
    type Error = Report<SeminarTerminPlanenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SeminarTerminGeplant, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;
        uow.seminare()
            .find_by_id(self.seminar_id.clone())
            .await
            .change_context(SeminarTerminPlanenFehler::SeminarNichtGefunden)?;

        uow.seminar_termine()
            .create(NeuerSeminarTermin::neu(
                self.seminar_id,
                self.zeitraum,
                self.ort,
                self.max_teilnehmer,
            ))
            .await
            .map(Versioned::into_data)
            .change_context(SeminarTerminPlanenFehler::Persistenz)
    }
}

#[derive(Clone)]
pub struct SeminarTerminAktualisieren {
    pub termin_id: SeminarTerminId,
    pub zeitraum: Zeitraum,
    pub ort: SeminarOrt,
    pub max_teilnehmer: Option<u32>,
}

#[derive(thiserror::Error, Debug)]
pub enum SeminarTerminAktualisierenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("termin nicht gefunden")]
    TerminNichtGefunden,
    #[error("termin ist nicht geplant")]
    NichtGeplant,
    #[error("aktualisierung verletzt invarianten")]
    Invariante,
}

#[async_trait]
impl UseCase<SeminarTermin> for SeminarTerminAktualisieren {
    type Error = Report<SeminarTerminAktualisierenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SeminarTermin, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;
        let mut termin = uow
            .seminar_termine()
            .find_by_id(self.termin_id)
            .await
            .change_context(SeminarTerminAktualisierenFehler::TerminNichtGefunden)?;

        // TODO: Benachrichtigung an Teilnehmer
        let SeminarTermin::Geplant(geplant) = termin.deref_mut() else {
            return Err(Report::new(SeminarTerminAktualisierenFehler::NichtGeplant));
        };
        geplant
            .aktualisieren(self.ort, self.zeitraum, self.max_teilnehmer)
            .change_context(SeminarTerminAktualisierenFehler::Invariante)?;

        uow.seminar_termine()
            .update(&mut termin)
            .await
            .change_context(SeminarTerminAktualisierenFehler::Persistenz)?;

        Ok(termin.into_data())
    }
}

#[derive(Clone)]
pub struct SeminarBuchungAnlegen {
    pub termin_id: SeminarTerminId,
    pub klient_id: KlientId,
    pub rabatt: Ratio,
}

#[derive(thiserror::Error, Debug)]
pub enum SeminarBuchungAnlegenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("termin nicht gefunden")]
    TerminNichtGefunden,
    #[error("klient nicht gefunden")]
    KlientNichtGefunden,
    #[error("termin ist nicht geplant")]
    NichtGeplant,
    #[error("buchung verletzt invarianten")]
    Invariante,
}

#[async_trait]
impl UseCase<SeminarTermin> for SeminarBuchungAnlegen {
    type Error = Report<SeminarBuchungAnlegenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SeminarTermin, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;
        uow.klienten()
            .find_by_id(self.klient_id.clone())
            .await
            .change_context(SeminarBuchungAnlegenFehler::KlientNichtGefunden)?;

        let mut termin = uow
            .seminar_termine()
            .find_by_id(self.termin_id)
            .await
            .change_context(SeminarBuchungAnlegenFehler::TerminNichtGefunden)?;

        let SeminarTermin::Geplant(geplant) = termin.deref_mut() else {
            return Err(Report::new(SeminarBuchungAnlegenFehler::NichtGeplant));
        };
        geplant
            .buchung_anlegen(self.klient_id, self.rabatt)
            .change_context(SeminarBuchungAnlegenFehler::Invariante)?;

        uow.seminar_termine()
            .update(&mut termin)
            .await
            .change_context(SeminarBuchungAnlegenFehler::Persistenz)?;

        Ok(termin.into_data())
    }
}

#[derive(Clone)]
pub struct SeminarBuchungStornieren {
    pub termin_id: SeminarTerminId,
    pub buchung_id: SeminarBuchungId,
}

#[derive(thiserror::Error, Debug)]
pub enum SeminarBuchungStornierenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("termin nicht gefunden")]
    TerminNichtGefunden,
    #[error("termin ist nicht geplant")]
    NichtGeplant,
    #[error("storno verletzt invarianten")]
    Invariante,
}

#[async_trait]
impl UseCase<SeminarTermin> for SeminarBuchungStornieren {
    type Error = Report<SeminarBuchungStornierenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SeminarTermin, Self::Error> {
        let now = ctx.clock().now();
        let ExecutionContext { uow, .. } = ctx;
        let mut termin = uow
            .seminar_termine()
            .find_by_id(self.termin_id)
            .await
            .change_context(SeminarBuchungStornierenFehler::TerminNichtGefunden)?;

        let SeminarTermin::Geplant(geplant) = termin.deref_mut() else {
            return Err(Report::new(SeminarBuchungStornierenFehler::NichtGeplant));
        };
        geplant
            .buchung_stornieren(&self.buchung_id, now)
            .change_context(SeminarBuchungStornierenFehler::Invariante)?;

        uow.seminar_termine()
            .update(&mut termin)
            .await
            .change_context(SeminarBuchungStornierenFehler::Persistenz)?;

        Ok(termin.into_data())
    }
}

#[derive(Clone)]
pub struct SeminarTerminAbsagen {
    pub termin_id: SeminarTerminId,
    pub grund: String,
}

#[derive(thiserror::Error, Debug)]
pub enum SeminarTerminAbsagenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("termin nicht gefunden")]
    TerminNichtGefunden,
    #[error("termin ist nicht geplant")]
    NichtGeplant,
}

#[async_trait]
impl UseCase<SeminarTermin> for SeminarTerminAbsagen {
    type Error = Report<SeminarTerminAbsagenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SeminarTermin, Self::Error> {
        let now = ctx.clock().now();
        let ExecutionContext { uow, .. } = ctx;
        let mut termin = uow
            .seminar_termine()
            .find_by_id(self.termin_id)
            .await
            .change_context(SeminarTerminAbsagenFehler::TerminNichtGefunden)?;

        let geplant = match termin.cloned_data() {
            SeminarTermin::Geplant(geplant) => geplant,
            _ => {
                return Err(Report::new(SeminarTerminAbsagenFehler::NichtGeplant));
            }
        };
        *termin.deref_mut() = SeminarTermin::from(geplant.absagen(self.grund, now));

        uow.seminar_termine()
            .update(&mut termin)
            .await
            .change_context(SeminarTerminAbsagenFehler::Persistenz)?;

        Ok(termin.into_data())
    }
}

#[derive(Clone)]
pub struct SeminarTerminAlsAbgehaltenMarkieren {
    pub termin_id: SeminarTerminId,
}

#[derive(thiserror::Error, Debug)]
pub enum SeminarTerminAlsAbgehaltenMarkierenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("termin nicht gefunden")]
    TerminNichtGefunden,
    #[error("seminar nicht gefunden")]
    SeminarNichtGefunden,
    #[error("termin ist nicht geplant")]
    NichtGeplant,
    #[error("leistungen-mapping unvollständig")]
    Invariante,
    #[error("klient nicht gefunden")]
    KlientNichtGefunden,
    #[error("pdf konnte nicht erzeugt werden")]
    Pdf,
    #[error("pdf konnte nicht gespeichert werden")]
    Speicher,
}

#[async_trait]
impl UseCase<SeminarTermin> for SeminarTerminAlsAbgehaltenMarkieren {
    type Error = Report<SeminarTerminAlsAbgehaltenMarkierenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<SeminarTermin, Self::Error> {
        let now = ctx.clock().now();
        let mut termin = ctx
            .uow
            .seminar_termine()
            .find_by_id(self.termin_id)
            .await
            .change_context(SeminarTerminAlsAbgehaltenMarkierenFehler::TerminNichtGefunden)?;

        let geplant = match termin.cloned_data() {
            SeminarTermin::Geplant(geplant) => geplant,
            _ => {
                return Err(Report::new(
                    SeminarTerminAlsAbgehaltenMarkierenFehler::NichtGeplant,
                ));
            }
        };

        let seminar = ctx
            .uow
            .seminare()
            .find_by_id(geplant.seminar_id().clone())
            .await
            .change_context(SeminarTerminAlsAbgehaltenMarkierenFehler::SeminarNichtGefunden)?
            .into_data();

        let mut mapping = FxHashMap::default();
        for (buchung_id, neue_leistung) in geplant.teilnahmeleistungen(&seminar) {
            let offen = ctx
                .uow
                .leistungen()
                .create(neue_leistung)
                .await
                .change_context(SeminarTerminAlsAbgehaltenMarkierenFehler::Persistenz)?;
            mapping.insert(buchung_id, offen.id().clone());
        }

        let abgehalten = geplant
            .als_abgehalten(now, mapping)
            .change_context(SeminarTerminAlsAbgehaltenMarkierenFehler::Invariante)?;
        *termin.deref_mut() = SeminarTermin::from(abgehalten);

        ctx.uow
            .seminar_termine()
            .update(&mut termin)
            .await
            .change_context(SeminarTerminAlsAbgehaltenMarkierenFehler::Persistenz)?;

        if let SeminarTermin::Abgehalten(abgehalten) = &*termin {
            for buchung in abgehalten.bestätigte_buchungen() {
                let klient = ctx
                    .uow
                    .klienten()
                    .find_by_id(buchung.klient_id().clone())
                    .await
                    .change_context(SeminarTerminAlsAbgehaltenMarkierenFehler::KlientNichtGefunden)?
                    .into_data();
                let dokument = teilnahme_dokument(abgehalten, &seminar, buchung, &klient);
                let pdf = ctx
                    .pdf_renderer()
                    .rendern(&dokument)
                    .await
                    .change_context(SeminarTerminAlsAbgehaltenMarkierenFehler::Pdf)?;
                ctx.object_store()
                    .put(&teilnahme_object_key(abgehalten.id(), buchung.id()), &pdf)
                    .await
                    .change_context(SeminarTerminAlsAbgehaltenMarkierenFehler::Speicher)?;
            }
        }

        Ok(termin.into_data())
    }
}

#[derive(Debug, Clone)]
pub struct BuchungUmsatz {
    pub buchung_id: SeminarBuchungId,
    pub klient_id: KlientId,
    pub netto: Preis,
    pub mwst: Preis,
    pub brutto: Preis,
}

#[derive(Debug, Clone)]
pub struct SeminarUmsatzVorschauErgebnis {
    pub termin_id: SeminarTerminId,
    pub seminar_id: SeminarId,
    pub teilnehmer_anzahl: u32,
    pub positionen: Vec<BuchungUmsatz>,
    pub gesamt_netto: Preis,
    pub gesamt_mwst: Preis,
    pub gesamt_brutto: Preis,
}

impl SeminarUmsatzVorschauErgebnis {
    fn aus_positionen(
        termin_id: SeminarTerminId,
        seminar_id: SeminarId,
        positionen: Vec<BuchungUmsatz>,
    ) -> Self {
        let gesamt_netto = positionen
            .iter()
            .fold(Preis::zero(), |acc, p| acc + p.netto.clone());
        let gesamt_mwst = positionen
            .iter()
            .fold(Preis::zero(), |acc, p| acc + p.mwst.clone());
        let gesamt_brutto = positionen
            .iter()
            .fold(Preis::zero(), |acc, p| acc + p.brutto.clone());
        Self {
            termin_id,
            seminar_id,
            teilnehmer_anzahl: positionen.len() as u32,
            positionen,
            gesamt_netto,
            gesamt_mwst,
            gesamt_brutto,
        }
    }
}

fn buchung_umsatz(
    seminar: &Seminar,
    buchung_id: SeminarBuchungId,
    klient_id: KlientId,
    rabatt: &Ratio,
) -> BuchungUmsatz {
    let netto = seminar.teilnahmegebühr_nach_rabatt(rabatt);
    let mwst = &netto * seminar.mwst();
    let brutto = netto.clone() + mwst.clone();
    BuchungUmsatz {
        buchung_id,
        klient_id,
        netto,
        mwst,
        brutto,
    }
}

#[derive(Clone)]
pub struct SeminarUmsatzVorschau {
    pub termin_id: SeminarTerminId,
}

#[derive(thiserror::Error, Debug)]
pub enum SeminarUmsatzVorschauFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("termin nicht gefunden")]
    TerminNichtGefunden,
    #[error("seminar nicht gefunden")]
    SeminarNichtGefunden,
    #[error("abgesagter termin hat keinen umsatz")]
    Abgesagt,
}

#[async_trait]
impl UseCase<SeminarUmsatzVorschauErgebnis> for SeminarUmsatzVorschau {
    type Error = Report<SeminarUmsatzVorschauFehler>;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> Result<SeminarUmsatzVorschauErgebnis, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;
        let termin = uow
            .seminar_termine()
            .find_by_id(self.termin_id)
            .await
            .change_context(SeminarUmsatzVorschauFehler::TerminNichtGefunden)?
            .into_data();

        umsatz_für_termin(&uow, &termin).await
    }
}

async fn umsatz_für_termin(
    uow: &crate::application::uow::UnitOfWork<'_>,
    termin: &SeminarTermin,
) -> Result<SeminarUmsatzVorschauErgebnis, Report<SeminarUmsatzVorschauFehler>> {
    let seminar = uow
        .seminare()
        .find_by_id(termin.seminar_id().clone())
        .await
        .change_context(SeminarUmsatzVorschauFehler::SeminarNichtGefunden)?
        .into_data();

    match termin {
        SeminarTermin::Abgesagt(_) => Err(Report::new(SeminarUmsatzVorschauFehler::Abgesagt)),
        SeminarTermin::Geplant(geplant) => {
            let positionen = geplant
                .bestätigte_buchungen()
                .map(|buchung| {
                    buchung_umsatz(
                        &seminar,
                        buchung.id().clone(),
                        buchung.klient_id().clone(),
                        buchung.rabatt(),
                    )
                })
                .collect();
            Ok(SeminarUmsatzVorschauErgebnis::aus_positionen(
                geplant.id().clone(),
                geplant.seminar_id().clone(),
                positionen,
            ))
        }
        SeminarTermin::Abgehalten(abgehalten) => {
            let mut positionen = Vec::new();
            for buchung in abgehalten.bestätigte_buchungen() {
                let Some(leistung_id) = abgehalten.leistung_fuer_buchung(buchung.id()) else {
                    continue;
                };
                let leistung = uow
                    .leistungen()
                    .find_by_id(leistung_id.clone())
                    .await
                    .change_context(SeminarUmsatzVorschauFehler::Persistenz)?
                    .into_data();
                let crate::domain::Leistung::Offen(offen) = leistung else {
                    continue;
                };
                let netto = offen.betrag();
                let mwst = &netto * offen.quelle().mwst();
                let brutto = netto.clone() + mwst.clone();
                positionen.push(BuchungUmsatz {
                    buchung_id: buchung.id().clone(),
                    klient_id: buchung.klient_id().clone(),
                    netto,
                    mwst,
                    brutto,
                });
            }
            Ok(SeminarUmsatzVorschauErgebnis::aus_positionen(
                abgehalten.id().clone(),
                abgehalten.seminar_id().clone(),
                positionen,
            ))
        }
    }
}

#[derive(Clone)]
pub struct SeminarUmsatzPrognoseBisDatum {
    pub stichtag: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct SeminarUmsatzPrognose {
    pub stichtag: NaiveDate,
    pub termine: Vec<SeminarUmsatzVorschauErgebnis>,
    pub gesamt_netto: Preis,
    pub gesamt_brutto: Preis,
}

#[derive(thiserror::Error, Debug)]
pub enum SeminarUmsatzPrognoseBisDatumFehler {
    #[error("persistenzfehler")]
    Persistenz,
}

#[async_trait]
impl UseCase<SeminarUmsatzPrognose> for SeminarUmsatzPrognoseBisDatum {
    type Error = Report<SeminarUmsatzPrognoseBisDatumFehler>;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> Result<SeminarUmsatzPrognose, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;
        let termine = uow
            .seminar_termine()
            .find_nicht_vollständig_abgerechnet_bis(self.stichtag)
            .await
            .change_context(SeminarUmsatzPrognoseBisDatumFehler::Persistenz)?;

        let mut ergebnisse = Vec::new();
        for termin in termine {
            match umsatz_für_termin(&uow, &termin).await {
                Ok(ergebnis) => ergebnisse.push(ergebnis),
                Err(err)
                    if matches!(err.current_context(), SeminarUmsatzVorschauFehler::Abgesagt) =>
                {
                    continue;
                }
                Err(err) => {
                    return Err(err.change_context(SeminarUmsatzPrognoseBisDatumFehler::Persistenz));
                }
            }
        }

        let gesamt_netto = ergebnisse
            .iter()
            .fold(Preis::zero(), |acc, t| acc + t.gesamt_netto.clone());
        let gesamt_brutto = ergebnisse
            .iter()
            .fold(Preis::zero(), |acc, t| acc + t.gesamt_brutto.clone());

        Ok(SeminarUmsatzPrognose {
            stichtag: self.stichtag,
            termine: ergebnisse,
            gesamt_netto,
            gesamt_brutto,
        })
    }
}
