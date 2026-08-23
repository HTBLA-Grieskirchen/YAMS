pub mod requests;

use std::sync::Arc;

use error_stack::ResultExt;
use uuid::Uuid;
use yams_core::{
    App, ResultReport,
    application::ExecutionError,
    domain::{HaustierId, KlientId},
    service::{
        BehandlungErstellen, HaustierErstellen, KlientErstellen, LeistungAusBehandlungBuchen,
        LeistungAusProduktBuchen, LeistungManuellErfassen, ProduktErstellen,
        TagesabschlussDurchfuehren,
    },
    uow::Versioned,
};

use crate::{
    requests::{
        BehandlungErstellung, HaustierErstellung, KlientErstellung,
        LeistungAusBehandlungErstellung, LeistungAusProduktErstellung,
        LeistungManuelleErstellung, ProduktErstellung, TagesabschlussErstellung,
    },
    schema::{
        Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung,
        schema_behandlung_from_domain, schema_haustier_from_domain, schema_klient_from_domain,
        schema_leistung_from_domain, schema_produkt_from_domain, schema_rechnung_from_domain,
        schema_rechnung_from_domain_rechnung,
    },
};

#[derive(Clone)]
pub struct YamsAppApi {
    app: Arc<App>,
}

impl YamsAppApi {
    pub fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }

    pub fn inner_app(&self) -> Arc<App> {
        self.app.clone()
    }
}

impl YamsAppApi {
    pub async fn klient_erstellen(
        &self,
        body: KlientErstellung,
    ) -> ResultReport<Klient, ExecutionError> {
        let klient = self
            .app
            .execute(KlientErstellen::try_from(body).change_context(ExecutionError)?)
            .await?;
        Ok(schema_klient_from_domain(klient, vec![]))
    }

    pub async fn haustier_erstellen(
        &self,
        body: HaustierErstellung,
    ) -> ResultReport<Haustier, ExecutionError> {
        let use_case = match HaustierErstellen::try_from(body) {
            Ok(use_case) => use_case,
            Err(error) => match error {},
        };
        let haustier = self.app.execute(use_case).await?;
        Ok(schema_haustier_from_domain(haustier))
    }

    pub async fn alle_haustiere(&self) -> ResultReport<Vec<Haustier>, ExecutionError> {
        let haustiere = self
            .app
            .execute_fn(async |ctx| ctx.uow.haustiere().find_all().await)
            .await?
            .into_iter()
            .map(Versioned::into_data);
        Ok(haustiere.map(schema_haustier_from_domain).collect())
    }

    pub async fn haustier_by_id(&self, id: Uuid) -> ResultReport<Haustier, ExecutionError> {
        let haustier = self
            .app
            .execute_fn(async |ctx| ctx.uow.haustiere().find_by_id(HaustierId(id)).await)
            .await?
            .into_data();
        Ok(schema_haustier_from_domain(haustier))
    }

    pub async fn produkt_erstellen(
        &self,
        body: ProduktErstellung,
    ) -> ResultReport<Produkt, ExecutionError> {
        let produkt = self
            .app
            .execute(ProduktErstellen::try_from(body).change_context(ExecutionError)?)
            .await?;
        Ok(schema_produkt_from_domain(produkt))
    }

    pub async fn behandlung_erstellen(
        &self,
        body: BehandlungErstellung,
    ) -> ResultReport<Behandlung, ExecutionError> {
        let behandlung = self
            .app
            .execute(BehandlungErstellen::try_from(body).change_context(ExecutionError)?)
            .await?;
        Ok(schema_behandlung_from_domain(behandlung))
    }

    pub async fn leistung_aus_produkt_buchen(
        &self,
        body: LeistungAusProduktErstellung,
    ) -> ResultReport<Leistung, ExecutionError> {
        let leistung = self
            .app
            .execute(
                LeistungAusProduktBuchen::try_from(body).change_context(ExecutionError)?,
            )
            .await?;
        Ok(schema_leistung_from_domain(leistung))
    }

    pub async fn leistung_aus_behandlung_buchen(
        &self,
        body: LeistungAusBehandlungErstellung,
    ) -> ResultReport<Leistung, ExecutionError> {
        let leistung = self
            .app
            .execute(
                LeistungAusBehandlungBuchen::try_from(body).change_context(ExecutionError)?,
            )
            .await?;
        Ok(schema_leistung_from_domain(leistung))
    }

    pub async fn leistung_manuell_erfassen(
        &self,
        body: LeistungManuelleErstellung,
    ) -> ResultReport<Leistung, ExecutionError> {
        let leistung = self
            .app
            .execute(
                LeistungManuellErfassen::try_from(body).change_context(ExecutionError)?,
            )
            .await?;
        Ok(schema_leistung_from_domain(leistung))
    }

    pub async fn tagesabschluss_durchfuehren(
        &self,
        body: TagesabschlussErstellung,
    ) -> ResultReport<Vec<Rechnung>, ExecutionError> {
        let rechnungen = self
            .app
            .execute(TagesabschlussDurchfuehren::from(body))
            .await?;
        Ok(rechnungen
            .into_iter()
            .map(schema_rechnung_from_domain)
            .collect())
    }

    pub async fn rechnungen_fuer_klient(
        &self,
        klient_id: Uuid,
    ) -> ResultReport<Vec<Rechnung>, ExecutionError> {
        let rechnungen = self
            .app
            .execute_fn(async |ctx| {
                ctx.uow
                    .rechnungen()
                    .find_by_klient_id(KlientId(klient_id))
                    .await
            })
            .await?
            .into_iter()
            .map(Versioned::into_data);
        Ok(rechnungen
            .map(schema_rechnung_from_domain_rechnung)
            .collect())
    }
}
