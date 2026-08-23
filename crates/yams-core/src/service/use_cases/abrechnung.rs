use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{Report, ResultExt};
use rust_decimal::Decimal;
use rustc_hash::FxHashMap;
use std::ops::DerefMut;

use crate::{
    application::uow::Versioned,
    domain::{
        Behandlung, BehandlungId, HaustierId, KlientId, Leistung, LeistungOffen,
        LeistungQuelle, Preis, Produkt, ProduktId, RechnungOffen, behandlung::NeueBehandlung,
        leistung::NeueLeistung, produkt::NeuesProdukt,
    },
    service::{ExecutionContext, UseCase},
};

#[derive(Clone)]
pub struct ProduktErstellen {
    pub name: String,
    pub beschreibung: String,
    pub einzelpreis: Preis,
    pub mwst_prozentsatz: Decimal,
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
            .create(NeuesProdukt {
                name: self.name,
                beschreibung: self.beschreibung,
                einzelpreis: self.einzelpreis,
                mwst_prozentsatz: self.mwst_prozentsatz,
            })
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
    pub mwst_prozentsatz: Decimal,
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
            .create(NeueBehandlung {
                name: self.name,
                beschreibung: self.beschreibung,
                standardpreis: self.standardpreis,
                mwst_prozentsatz: self.mwst_prozentsatz,
            })
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
    pub menge: Decimal,
    pub leistungsdatum: NaiveDate,
}

#[derive(thiserror::Error, Debug)]
pub enum LeistungAusProduktBuchenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("produkt nicht gefunden")]
    ProduktNichtGefunden,
    #[error("ungültiger betrag")]
    UngültigerBetrag,
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

        produkt
            .einzelpreis
            .multiply(self.menge)
            .change_context(LeistungAusProduktBuchenFehler::UngültigerBetrag)?;

        uow.leistungen()
            .create(NeueLeistung {
                klient_id: self.klient_id,
                haustier_id: self.haustier_id,
                beschreibung: produkt.name.clone(),
                leistungsdatum: self.leistungsdatum,
                quelle: LeistungQuelle::Produkt {
                    produkt_id: self.produkt_id,
                    menge: self.menge,
                    einzelpreis: produkt.einzelpreis.clone(),
                    mwst_prozentsatz: produkt.mwst_prozentsatz,
                },
            })
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
            .unwrap_or_else(|| behandlung.standardpreis.clone());

        uow.leistungen()
            .create(NeueLeistung {
                klient_id: self.klient_id,
                haustier_id: self.haustier_id,
                beschreibung: behandlung.name.clone(),
                leistungsdatum: self.leistungsdatum,
                quelle: LeistungQuelle::Behandlung {
                    behandlung_id: self.behandlung_id,
                    preis,
                    mwst_prozentsatz: behandlung.mwst_prozentsatz,
                },
            })
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
    pub mwst_prozentsatz: Decimal,
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
            .create(NeueLeistung {
                klient_id: self.klient_id,
                haustier_id: self.haustier_id,
                beschreibung: self.beschreibung,
                leistungsdatum: self.leistungsdatum,
                quelle: LeistungQuelle::Manuell {
                    preis: self.betrag,
                    mwst_prozentsatz: self.mwst_prozentsatz,
                },
            })
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
}

#[async_trait]
impl UseCase<Vec<RechnungOffen>> for TagesabschlussDurchführen {
    type Error = Report<TagesabschlussDurchführenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Vec<RechnungOffen>, Self::Error> {
        let abschlussdatum = match self.abschlussdatum {
            Some(datum) => datum,
            None => ctx.clock().today(),
        };
        let ExecutionContext { mut uow, .. } = ctx;

        let leistungen = uow
            .leistungen()
            .find_offene_by_datum(abschlussdatum)
            .await
            .change_context(TagesabschlussDurchführenFehler::Persistenz)?;

        let mut gruppen: FxHashMap<KlientId, Vec<Versioned<LeistungOffen>>> =
            FxHashMap::default();
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

            let rechnungsnummer = uow
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
                    klient_id,
                    rechnungsnummer,
                    abschlussdatum,
                    &mut leistung_refs,
                )
                .map_err(|report| report.change_context(TagesabschlussDurchführenFehler::Rechnung))?
            };

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

            uow.checkpoint()
                .await
                .change_context(TagesabschlussDurchführenFehler::Persistenz)?;

            rechnungen.push(persisted.into_data());
        }

        Ok(rechnungen)
    }
}
