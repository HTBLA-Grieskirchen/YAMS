use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{Report, ResultExt};
use rustc_hash::FxHashMap;
use std::ops::DerefMut;

use crate::{
    application::uow::Versioned,
    domain::{
        Behandlung, BehandlungId, HaustierId, KlientId, Leistung, LeistungOffen, LeistungQuelle,
        Menge, Preis, Produkt, ProduktId, Ratio, RechnungOffen, behandlung::NeueBehandlung,
        leistung::NeueLeistung, produkt::NeuesProdukt,
    },
    ports::rechnung_object_key,
    service::{ExecutionContext, UseCase, pdf::rechnungsdokument},
};

#[derive(Clone)]
pub struct ProduktErstellen {
    pub name: String,
    pub beschreibung: String,
    pub einzelpreis: Preis,
    pub mwst: Ratio,
}

#[derive(thiserror::Error, Debug)]
pub enum ProduktErstellenFehler {
    #[error("fehler beim anlegen des produkts")]
    Erstellung,
}

#[async_trait]
impl UseCase<Produkt> for ProduktErstellen {
    type Error = Report<ProduktErstellenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Produkt, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;

        uow.produkte()
            .create(NeuesProdukt::neu(
                self.name,
                self.beschreibung,
                self.einzelpreis,
                self.mwst,
            ))
            .await
            .map(Versioned::into_data)
            .change_context(ProduktErstellenFehler::Erstellung)
    }
}

#[derive(Clone)]
pub struct BehandlungErstellen {
    pub name: String,
    pub beschreibung: String,
    pub standardpreis: Preis,
    pub mwst: Ratio,
}

#[derive(thiserror::Error, Debug)]
pub enum BehandlungErstellenFehler {
    #[error("fehler beim anlegen der behandlung")]
    Erstellung,
}

#[async_trait]
impl UseCase<Behandlung> for BehandlungErstellen {
    type Error = Report<BehandlungErstellenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Behandlung, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;

        uow.behandlungen()
            .create(NeueBehandlung::neu(
                self.name,
                self.beschreibung,
                self.standardpreis,
                self.mwst,
            ))
            .await
            .map(Versioned::into_data)
            .change_context(BehandlungErstellenFehler::Erstellung)
    }
}

#[derive(Clone)]
pub struct LeistungAusProduktBuchen {
    pub produkt_id: ProduktId,
    pub klient_id: KlientId,
    pub haustier_id: Option<HaustierId>,
    pub menge: Menge,
    pub leistungsdatum: NaiveDate,
}

#[derive(thiserror::Error, Debug)]
pub enum LeistungAusProduktBuchenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("produkt nicht gefunden")]
    ProduktNichtGefunden,
}

#[async_trait]
impl UseCase<LeistungOffen> for LeistungAusProduktBuchen {
    type Error = Report<LeistungAusProduktBuchenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<LeistungOffen, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;

        let produkt = uow
            .produkte()
            .find_by_id(self.produkt_id.clone())
            .await
            .change_context(LeistungAusProduktBuchenFehler::ProduktNichtGefunden)?;

        uow.leistungen()
            .create(NeueLeistung::neu(
                self.klient_id,
                self.haustier_id,
                produkt.name(),
                self.leistungsdatum,
                LeistungQuelle::Produkt {
                    produkt_id: self.produkt_id,
                    menge: self.menge,
                    einzelpreis: produkt.einzelpreis().clone(),
                    mwst: produkt.mwst().clone(),
                },
            ))
            .await
            .map(Versioned::into_data)
            .change_context(LeistungAusProduktBuchenFehler::Persistenz)
    }
}

#[derive(Clone)]
pub struct LeistungAusBehandlungBuchen {
    pub behandlung_id: BehandlungId,
    pub klient_id: KlientId,
    pub haustier_id: Option<HaustierId>,
    pub leistungsdatum: NaiveDate,
    pub preis_override: Option<Preis>,
}

#[derive(thiserror::Error, Debug)]
pub enum LeistungAusBehandlungBuchenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("behandlung nicht gefunden")]
    BehandlungNichtGefunden,
}

#[async_trait]
impl UseCase<LeistungOffen> for LeistungAusBehandlungBuchen {
    type Error = Report<LeistungAusBehandlungBuchenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<LeistungOffen, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;

        let behandlung = uow
            .behandlungen()
            .find_by_id(self.behandlung_id.clone())
            .await
            .change_context(LeistungAusBehandlungBuchenFehler::BehandlungNichtGefunden)?;

        let preis = self
            .preis_override
            .unwrap_or_else(|| behandlung.standardpreis().clone());

        uow.leistungen()
            .create(NeueLeistung::neu(
                self.klient_id,
                self.haustier_id,
                behandlung.name(),
                self.leistungsdatum,
                LeistungQuelle::Behandlung {
                    behandlung_id: self.behandlung_id,
                    preis,
                    mwst: behandlung.mwst().clone(),
                },
            ))
            .await
            .map(Versioned::into_data)
            .change_context(LeistungAusBehandlungBuchenFehler::Persistenz)
    }
}

#[derive(Clone)]
pub struct LeistungManuellErfassen {
    pub klient_id: KlientId,
    pub haustier_id: Option<HaustierId>,
    pub beschreibung: String,
    pub betrag: Preis,
    pub mwst: Ratio,
    pub leistungsdatum: NaiveDate,
}

#[derive(thiserror::Error, Debug)]
pub enum LeistungManuellErfassenFehler {
    #[error("persistenzfehler")]
    Persistenz,
}

#[async_trait]
impl UseCase<LeistungOffen> for LeistungManuellErfassen {
    type Error = Report<LeistungManuellErfassenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<LeistungOffen, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;

        uow.leistungen()
            .create(NeueLeistung::neu(
                self.klient_id,
                self.haustier_id,
                self.beschreibung,
                self.leistungsdatum,
                LeistungQuelle::Manuell {
                    preis: self.betrag,
                    mwst: self.mwst,
                },
            ))
            .await
            .map(Versioned::into_data)
            .change_context(LeistungManuellErfassenFehler::Persistenz)
    }
}

#[derive(Clone)]
pub struct TagesabschlussDurchführen {
    pub abschlussdatum: Option<NaiveDate>,
}

#[derive(thiserror::Error, Debug)]
pub enum TagesabschlussDurchführenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("rechnung konnte nicht erstellt werden")]
    Rechnung,
    #[error("klient nicht gefunden")]
    KlientNichtGefunden,
    #[error("pdf konnte nicht erzeugt werden")]
    Pdf,
    #[error("pdf konnte nicht gespeichert werden")]
    Speicher,
}

#[async_trait]
impl UseCase<Vec<RechnungOffen>> for TagesabschlussDurchführen {
    type Error = Report<TagesabschlussDurchführenFehler>;

    async fn perform(
        self,
        mut ctx: ExecutionContext<'_>,
    ) -> Result<Vec<RechnungOffen>, Self::Error> {
        let abschlussdatum = match self.abschlussdatum {
            Some(datum) => datum,
            None => ctx.clock().today(),
        };

        let leistungen = ctx
            .uow
            .leistungen()
            .find_offene_by_datum(abschlussdatum)
            .await
            .change_context(TagesabschlussDurchführenFehler::Persistenz)?;

        let mut gruppen: FxHashMap<KlientId, Vec<Versioned<LeistungOffen>>> = FxHashMap::default();
        for leistung in leistungen {
            gruppen
                .entry(leistung.klient_id().clone())
                .or_default()
                .push(leistung);
        }

        let mut rechnungen = Vec::new();
        for (klient_id, gruppen_leistungen) in gruppen {
            let mut versioned_leistungen: Vec<Versioned<Leistung>> = gruppen_leistungen
                .into_iter()
                .map(|l| Versioned::new(l.v(), Leistung::from(l.cloned_data())))
                .collect();

            let rechnungsnummer = ctx
                .uow
                .rechnungen()
                .nächste_rechnungsnummer()
                .await
                .change_context(TagesabschlussDurchführenFehler::Persistenz)?;

            let rechnung = {
                let mut leistung_refs: Vec<&mut Leistung> = versioned_leistungen
                    .iter_mut()
                    .map(DerefMut::deref_mut)
                    .collect();

                RechnungOffen::aus_leistungen(
                    klient_id.clone(),
                    rechnungsnummer,
                    abschlussdatum,
                    &mut leistung_refs,
                )
                .map_err(|report| {
                    report.change_context(TagesabschlussDurchführenFehler::Rechnung)
                })?
            };

            let persisted = ctx
                .uow
                .rechnungen()
                .create(rechnung)
                .await
                .change_context(TagesabschlussDurchführenFehler::Persistenz)?;

            for mut versioned in versioned_leistungen {
                if matches!(*versioned, Leistung::Abgerechnet(_)) {
                    ctx.uow
                        .leistungen()
                        .update(&mut versioned)
                        .await
                        .change_context(TagesabschlussDurchführenFehler::Persistenz)?;
                }
            }

            let klient = ctx
                .uow
                .klienten()
                .find_by_id(klient_id)
                .await
                .change_context(TagesabschlussDurchführenFehler::KlientNichtGefunden)?
                .into_data();
            let dokument = rechnungsdokument(&*persisted, &klient);
            let pdf = ctx
                .pdf_renderer()
                .rendern(&dokument)
                .await
                .change_context(TagesabschlussDurchführenFehler::Pdf)?;
            ctx.object_store()
                .put(&rechnung_object_key(persisted.id()), &pdf)
                .await
                .change_context(TagesabschlussDurchführenFehler::Speicher)?;

            ctx.uow
                .checkpoint()
                .await
                .change_context(TagesabschlussDurchführenFehler::Persistenz)?;

            rechnungen.push(persisted.into_data());
        }

        Ok(rechnungen)
    }
}
