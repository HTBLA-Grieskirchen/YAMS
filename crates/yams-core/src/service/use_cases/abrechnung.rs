use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{Report, ResultExt};
use rustc_hash::FxHashMap;
use std::ops::DerefMut;
use tracing::{debug, info};

use crate::{
    application::uow::Versioned,
    domain::{
        Behandlung, BehandlungId, HaustierId, KlientId, Leistung, LeistungOffen, LeistungQuelle,
        Menge, Preis, Produkt, ProduktId, Ratio, RechnungOffen, behandlung::NeueBehandlung,
        leistung::NeueLeistung, produkt::NeuesProdukt,
    },
    service::{
        ExecutionContext, UseCase,
        pdf::{
            mit_objekt_rollback, nach_pdf_persistieren, pdfs_rendern_und_ablegen,
            rechnung_object_key, rechnungsdokument,
        },
    },
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
        let uow = ctx
            .enter()
            .await
            .change_context(ProduktErstellenFehler::Erstellung)?;

        let result = uow
            .produkte()
            .create(NeuesProdukt::neu(
                self.name,
                self.beschreibung,
                self.einzelpreis,
                self.mwst,
            ))
            .await
            .map(Versioned::into_data)
            .change_context(ProduktErstellenFehler::Erstellung);

        let produkt = uow.finish(result, ProduktErstellenFehler::Erstellung).await?;
        debug!(id = ?produkt.id(), name = produkt.name(), "produkt angelegt");
        Ok(produkt)
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
        let uow = ctx
            .enter()
            .await
            .change_context(BehandlungErstellenFehler::Erstellung)?;

        let result = uow
            .behandlungen()
            .create(NeueBehandlung::neu(
                self.name,
                self.beschreibung,
                self.standardpreis,
                self.mwst,
            ))
            .await
            .map(Versioned::into_data)
            .change_context(BehandlungErstellenFehler::Erstellung);

        let behandlung = uow
            .finish(result, BehandlungErstellenFehler::Erstellung)
            .await?;
        debug!(id = ?behandlung.id(), name = behandlung.name(), "behandlung angelegt");
        Ok(behandlung)
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
        let uow = ctx
            .enter()
            .await
            .change_context(LeistungAusProduktBuchenFehler::Persistenz)?;

        let result = async {
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
        .await;

        let leistung = uow
            .finish(result, LeistungAusProduktBuchenFehler::Persistenz)
            .await?;
        info!(
            id = ?leistung.id(),
            klient_id = ?leistung.klient_id(),
            "leistung aus produkt gebucht"
        );
        Ok(leistung)
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
        let uow = ctx
            .enter()
            .await
            .change_context(LeistungAusBehandlungBuchenFehler::Persistenz)?;

        let result = async {
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
        .await;

        let leistung = uow
            .finish(result, LeistungAusBehandlungBuchenFehler::Persistenz)
            .await?;
        info!(
            id = ?leistung.id(),
            klient_id = ?leistung.klient_id(),
            "leistung aus behandlung gebucht"
        );
        Ok(leistung)
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
        let uow = ctx
            .enter()
            .await
            .change_context(LeistungManuellErfassenFehler::Persistenz)?;

        let result = uow
            .leistungen()
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
            .change_context(LeistungManuellErfassenFehler::Persistenz);

        let leistung = uow
            .finish(result, LeistungManuellErfassenFehler::Persistenz)
            .await?;
        info!(
            id = ?leistung.id(),
            klient_id = ?leistung.klient_id(),
            "leistung manuell erfasst"
        );
        Ok(leistung)
    }
}

#[derive(Clone)]
pub struct TagesabschlussDurchführen {
    pub abschlussdatum: Option<NaiveDate>,
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
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

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Vec<RechnungOffen>, Self::Error> {
        let abschlussdatum = match self.abschlussdatum {
            Some(datum) => datum,
            None => ctx.clock().today(),
        };
        info!(%abschlussdatum, "tagesabschluss gestartet");

        let uow = ctx
            .enter()
            .await
            .change_context(TagesabschlussDurchführenFehler::Persistenz)?;
        let leistungen = {
            let result = uow
                .leistungen()
                .find_offene_by_datum(abschlussdatum)
                .await
                .change_context(TagesabschlussDurchführenFehler::Persistenz);
            uow.finish(result, TagesabschlussDurchführenFehler::Persistenz)
                .await?
        };
        info!(
            %abschlussdatum,
            leistungen = leistungen.len(),
            "offene leistungen geladen"
        );

        let mut gruppen: FxHashMap<KlientId, Vec<Versioned<LeistungOffen>>> = FxHashMap::default();
        for leistung in leistungen {
            gruppen
                .entry(leistung.klient_id().clone())
                .or_default()
                .push(leistung);
        }
        info!(%abschlussdatum, klienten = gruppen.len(), "rechnungsgruppen gebildet");

        let mut rechnungen = Vec::new();
        for (klient_id, gruppen_leistungen) in gruppen {
            let versioned_leistungen: Vec<Versioned<Leistung>> = gruppen_leistungen
                .into_iter()
                .map(|l| Versioned::new(l.v(), Leistung::from(l.cloned_data())))
                .collect();

            let uow = ctx
                .enter()
                .await
                .change_context(TagesabschlussDurchführenFehler::Persistenz)?;

            let result = async {
                let rechnungsnummer = uow
                    .rechnungen()
                    .nächste_rechnungsnummer()
                    .await
                    .change_context(TagesabschlussDurchführenFehler::Persistenz)?;

                let mut versioned_leistungen = versioned_leistungen;
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

                let klient = uow
                    .klienten()
                    .find_by_id(klient_id)
                    .await
                    .change_context(TagesabschlussDurchführenFehler::KlientNichtGefunden)?
                    .into_data();
                let dokument = rechnungsdokument(&rechnung, &klient);
                let object_key = rechnung_object_key(rechnung.id());
                Ok((rechnung, versioned_leistungen, dokument, object_key))
            }
            .await;

            let (rechnung, versioned_leistungen, dokument, object_key) = uow
                .finish(result, TagesabschlussDurchführenFehler::Persistenz)
                .await?;

            let stored = pdfs_rendern_und_ablegen(
                ctx.pdf_renderer(),
                ctx.object_store(),
                vec![(object_key, dokument)],
                TagesabschlussDurchführenFehler::Pdf,
                TagesabschlussDurchführenFehler::Speicher,
            )
            .await?;

            let uow = mit_objekt_rollback(
                ctx.object_store(),
                &stored,
                ctx.enter()
                    .await
                    .change_context(TagesabschlussDurchführenFehler::Persistenz),
            )
            .await?;

            let result = async {
                let persisted = uow
                    .rechnungen()
                    .create(rechnung)
                    .await
                    .change_context(TagesabschlussDurchführenFehler::Persistenz)?;

                for mut versioned in versioned_leistungen {
                    if matches!(*versioned, Leistung::Abgerechnet(_)) {
                        uow.leistungen()
                            .update(&mut versioned)
                            .await
                            .change_context(TagesabschlussDurchführenFehler::Persistenz)?;
                    }
                }

                Ok(persisted)
            }
            .await;

            let persisted = nach_pdf_persistieren(
                uow,
                result,
                TagesabschlussDurchführenFehler::Persistenz,
                ctx.object_store(),
                &stored,
            )
            .await?;

            let rechnung = persisted.into_data();
            info!(
                %abschlussdatum,
                klient_id = ?rechnung.klient_id(),
                rechnungsnummer = rechnung.rechnungsnummer(),
                "rechnung erstellt"
            );
            rechnungen.push(rechnung);
        }

        info!(
            %abschlussdatum,
            rechnungen = rechnungen.len(),
            "tagesabschluss abgeschlossen"
        );
        Ok(rechnungen)
    }
}
