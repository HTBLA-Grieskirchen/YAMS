use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{IntoReport, Report, ResultExt};
use rust_decimal::Decimal;
use rustc_hash::FxHashMap;

use crate::{
    application::uow::Versioned,
    domain::{
        Behandlung, BehandlungId, HaustierId, KlientId, Leistung, LeistungQuelle, Preis, Produkt,
        ProduktId, Rechnung, behandlung::NeueBehandlung, leistung::NeueLeistung,
        produkt::NeuesProdukt, rechnung::RechnungFehler,
    },
    service::{ExecutionContext, UseCase},
};

#[derive(Clone)]
pub struct ProduktErstellen {
    pub name: String,
    pub beschreibung: String,
    pub einzelpreis: Preis,
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
            })
            .await
            .map(Versioned::into_data)
            .map_err(IntoReport::into_report)
            .change_context(ProduktErstellenFehler::Erstellung)
    }
}

#[derive(Clone)]
pub struct BehandlungErstellen {
    pub name: String,
    pub beschreibung: String,
    pub standardpreis: Preis,
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
            })
            .await
            .map(Versioned::into_data)
            .map_err(IntoReport::into_report)
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
    UngueltigerBetrag,
}

#[async_trait]
impl UseCase<Leistung> for LeistungAusProduktBuchen {
    type Error = Report<LeistungAusProduktBuchenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Leistung, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;

        let produkt = uow
            .produkte()
            .find_by_id(self.produkt_id.clone())
            .await
            .change_context(LeistungAusProduktBuchenFehler::ProduktNichtGefunden)?;

        let betrag = produkt
            .einzelpreis
            .multiply(self.menge)
            .change_context(LeistungAusProduktBuchenFehler::UngueltigerBetrag)?;

        uow.leistungen()
            .create(NeueLeistung {
                klient_id: self.klient_id,
                haustier_id: self.haustier_id,
                beschreibung: produkt.name.clone(),
                betrag,
                leistungsdatum: self.leistungsdatum,
                quelle: LeistungQuelle::Produkt(self.produkt_id),
            })
            .await
            .map(Versioned::into_data)
            .map_err(IntoReport::into_report)
            .change_context(LeistungAusProduktBuchenFehler::Persistenz)
    }
}

#[derive(Clone)]
pub struct LeistungAusBehandlungBuchen {
    pub behandlung_id: BehandlungId,
    pub klient_id: KlientId,
    pub haustier_id: Option<HaustierId>,
    pub leistungsdatum: NaiveDate,
}

#[derive(thiserror::Error, Debug)]
pub enum LeistungAusBehandlungBuchenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("behandlung nicht gefunden")]
    BehandlungNichtGefunden,
}

#[async_trait]
impl UseCase<Leistung> for LeistungAusBehandlungBuchen {
    type Error = Report<LeistungAusBehandlungBuchenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Leistung, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;

        let behandlung = uow
            .behandlungen()
            .find_by_id(self.behandlung_id.clone())
            .await
            .change_context(LeistungAusBehandlungBuchenFehler::BehandlungNichtGefunden)?;

        uow.leistungen()
            .create(NeueLeistung {
                klient_id: self.klient_id,
                haustier_id: self.haustier_id,
                beschreibung: behandlung.name.clone(),
                betrag: behandlung.standardpreis.clone(),
                leistungsdatum: self.leistungsdatum,
                quelle: LeistungQuelle::Behandlung(self.behandlung_id),
            })
            .await
            .map(Versioned::into_data)
            .map_err(IntoReport::into_report)
            .change_context(LeistungAusBehandlungBuchenFehler::Persistenz)
    }
}

#[derive(Clone)]
pub struct LeistungManuellErfassen {
    pub klient_id: KlientId,
    pub haustier_id: Option<HaustierId>,
    pub beschreibung: String,
    pub betrag: Preis,
    pub leistungsdatum: NaiveDate,
}

#[derive(thiserror::Error, Debug)]
pub enum LeistungManuellErfassenFehler {
    #[error("persistenzfehler")]
    Persistenz,
}

#[async_trait]
impl UseCase<Leistung> for LeistungManuellErfassen {
    type Error = Report<LeistungManuellErfassenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Leistung, Self::Error> {
        let ExecutionContext { uow, .. } = ctx;

        uow.leistungen()
            .create(NeueLeistung {
                klient_id: self.klient_id,
                haustier_id: self.haustier_id,
                beschreibung: self.beschreibung,
                betrag: self.betrag,
                leistungsdatum: self.leistungsdatum,
                quelle: LeistungQuelle::Manuell,
            })
            .await
            .map(Versioned::into_data)
            .map_err(IntoReport::into_report)
            .change_context(LeistungManuellErfassenFehler::Persistenz)
    }
}

#[derive(Clone)]
pub struct TagesabschlussDurchfuehren {
    pub abschlussdatum: Option<NaiveDate>,
}

#[derive(thiserror::Error, Debug)]
pub enum TagesabschlussDurchfuehrenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("rechnung konnte nicht erstellt werden")]
    RechnungErstellung(RechnungFehler),
}

#[async_trait]
impl UseCase<Vec<Rechnung>> for TagesabschlussDurchfuehren {
    type Error = Report<TagesabschlussDurchfuehrenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Vec<Rechnung>, Self::Error> {
        let abschlussdatum = match self.abschlussdatum {
            Some(datum) => datum,
            None => ctx.clock().today(),
        };
        let ExecutionContext { mut uow, .. } = ctx;

        let leistungen = uow
            .leistungen()
            .find_offene_by_datum(abschlussdatum)
            .await
            .change_context(TagesabschlussDurchfuehrenFehler::Persistenz)?;

        let mut gruppen: FxHashMap<KlientId, Vec<Versioned<Leistung>>> = FxHashMap::default();
        for leistung in leistungen {
            gruppen
                .entry(leistung.klient_id.clone())
                .or_default()
                .push(leistung);
        }

        let mut rechnungen = Vec::new();
        for (klient_id, gruppen_leistungen) in gruppen {
            let leistung_daten = gruppen_leistungen
                .iter()
                .map(|l| l.cloned_data())
                .collect::<Vec<_>>();

            let rechnungsnummer = uow
                .rechnungen()
                .naechste_rechnungsnummer()
                .await
                .change_context(TagesabschlussDurchfuehrenFehler::Persistenz)?;

            let rechnung = Rechnung::aus_leistungen(
                klient_id,
                rechnungsnummer,
                abschlussdatum,
                leistung_daten,
            )
            .map_err(|e| Report::new(TagesabschlussDurchfuehrenFehler::RechnungErstellung(e)))?;

            let persisted = uow
                .rechnungen()
                .create(rechnung)
                .await
                .change_context(TagesabschlussDurchfuehrenFehler::Persistenz)?;

            for leistung in gruppen_leistungen {
                uow.leistungen()
                    .mark_abgerechnet(leistung.id.clone(), persisted.id.clone())
                    .await
                    .change_context(TagesabschlussDurchfuehrenFehler::Persistenz)?;
            }

            uow.checkpoint()
                .await
                .change_context(TagesabschlussDurchfuehrenFehler::Persistenz)?;

            rechnungen.push(persisted.into_data());
        }

        Ok(rechnungen)
    }
}
