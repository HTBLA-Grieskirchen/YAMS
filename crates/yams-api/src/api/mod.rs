pub mod requests;

use std::sync::Arc;

use error_stack::{Report, ResultExt};
use http::StatusCode;
use uuid::Uuid;
use yams_core::{
    App, ResultReport,
    application::ExecutionError,
    domain::{
        HaustierId, KlientId, RechnungId, SeminarBuchungId, SeminarId,
        SeminarTermin as DomainSeminarTermin, SeminarTerminId,
    },
    service::{
        BehandlungErstellen, HaustierErstellen, KlientErstellen, LeistungAusBehandlungBuchen,
        LeistungAusProduktBuchen, LeistungManuellErfassen, ProduktErstellen,
        SeminarBuchungStornieren, SeminarErstellen, SeminarTerminPlanen,
        SeminarUmsatzPrognoseBisDatum, TagesabschlussDurchführen, rechnung_pdf_laden,
        teilnahme_pdf_laden,
    },
    uow::Versioned,
};

use crate::{
    requests::{
        BehandlungErstellung, HaustierErstellung, KlientErstellung,
        LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung,
        ProduktErstellung, SeminarBuchungErstellung, SeminarErstellung, SeminarTerminAbsage,
        SeminarTerminAktualisierung, SeminarTerminErstellung, TagesabschlussErstellung,
        abgehalten_use_case, buchung_id,
    },
    schema::{
        Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung, Seminar, SeminarTermin,
        SeminarUmsatzPrognose, SeminarUmsatzVorschau, schema_behandlung_from_domain,
        schema_haustier_from_domain, schema_klient_from_domain, schema_leistung_from_domain,
        schema_produkt_from_domain, schema_prognose_from_domain, schema_rechnung_from_domain,
        schema_rechnung_from_domain_rechnung, schema_seminar_from_domain,
        schema_seminar_termin_from_domain, schema_umsatz_from_domain,
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
            .execute(LeistungAusProduktBuchen::try_from(body).change_context(ExecutionError)?)
            .await?;
        Ok(schema_leistung_from_domain(leistung))
    }

    pub async fn leistung_aus_behandlung_buchen(
        &self,
        body: LeistungAusBehandlungErstellung,
    ) -> ResultReport<Leistung, ExecutionError> {
        let leistung = self
            .app
            .execute(LeistungAusBehandlungBuchen::try_from(body).change_context(ExecutionError)?)
            .await?;
        Ok(schema_leistung_from_domain(leistung))
    }

    pub async fn leistung_manuell_erfassen(
        &self,
        body: LeistungManuelleErstellung,
    ) -> ResultReport<Leistung, ExecutionError> {
        let leistung = self
            .app
            .execute(LeistungManuellErfassen::try_from(body).change_context(ExecutionError)?)
            .await?;
        Ok(schema_leistung_from_domain(leistung))
    }

    pub async fn tagesabschluss_durchführen(
        &self,
        body: TagesabschlussErstellung,
    ) -> ResultReport<Vec<Rechnung>, ExecutionError> {
        let rechnungen = self
            .app
            .execute(TagesabschlussDurchführen::from(body))
            .await?;
        Ok(rechnungen
            .into_iter()
            .map(schema_rechnung_from_domain)
            .collect())
    }

    pub async fn rechnungen_für_klient(
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

    pub async fn rechnung_pdf(&self, rechnung_id: Uuid) -> ResultReport<Vec<u8>, ExecutionError> {
        let pdf = self
            .app
            .execute_fn(async move |ctx| {
                rechnung_pdf_laden(ctx.object_store(), &RechnungId(rechnung_id)).await
            })
            .await?;
        match pdf {
            Some(bytes) => Ok(bytes),
            None => Err(Report::new(ExecutionError)
                .attach("pdf not found")
                .attach_opaque(StatusCode::NOT_FOUND)),
        }
    }

    pub async fn teilnahmebestätigung_pdf(
        &self,
        termin_id: Uuid,
        buchung_id: Uuid,
    ) -> ResultReport<Vec<u8>, ExecutionError> {
        let pdf = self
            .app
            .execute_fn(async move |ctx| {
                teilnahme_pdf_laden(
                    ctx.object_store(),
                    &SeminarTerminId(termin_id),
                    &SeminarBuchungId(buchung_id),
                )
                .await
            })
            .await?;
        match pdf {
            Some(bytes) => Ok(bytes),
            None => Err(Report::new(ExecutionError)
                .attach("pdf not found")
                .attach_opaque(StatusCode::NOT_FOUND)),
        }
    }

    pub async fn seminar_erstellen(
        &self,
        body: SeminarErstellung,
    ) -> ResultReport<Seminar, ExecutionError> {
        let seminar = self
            .app
            .execute(SeminarErstellen::try_from(body).change_context(ExecutionError)?)
            .await?;
        Ok(schema_seminar_from_domain(seminar))
    }

    pub async fn seminar_by_id(&self, id: Uuid) -> ResultReport<Seminar, ExecutionError> {
        let seminar = self
            .app
            .execute_fn(async |ctx| ctx.uow.seminare().find_by_id(SeminarId(id)).await)
            .await?
            .into_data();
        Ok(schema_seminar_from_domain(seminar))
    }

    pub async fn seminar_termin_planen(
        &self,
        body: SeminarTerminErstellung,
    ) -> ResultReport<SeminarTermin, ExecutionError> {
        let termin = self
            .app
            .execute(SeminarTerminPlanen::try_from(body).change_context(ExecutionError)?)
            .await?;
        Ok(schema_seminar_termin_from_domain(
            DomainSeminarTermin::from(termin),
        ))
    }

    pub async fn seminar_termin_by_id(
        &self,
        id: Uuid,
    ) -> ResultReport<SeminarTermin, ExecutionError> {
        let termin = self
            .app
            .execute_fn(async |ctx| {
                ctx.uow
                    .seminar_termine()
                    .find_by_id(SeminarTerminId(id))
                    .await
            })
            .await?
            .into_data();
        Ok(schema_seminar_termin_from_domain(termin))
    }

    pub async fn seminar_termin_aktualisieren(
        &self,
        id: Uuid,
        body: SeminarTerminAktualisierung,
    ) -> ResultReport<SeminarTermin, ExecutionError> {
        let termin = self
            .app
            .execute(body.into_use_case(id).change_context(ExecutionError)?)
            .await?;
        Ok(schema_seminar_termin_from_domain(termin))
    }

    pub async fn seminar_buchung_anlegen(
        &self,
        termin_id: Uuid,
        body: SeminarBuchungErstellung,
    ) -> ResultReport<SeminarTermin, ExecutionError> {
        let termin = self
            .app
            .execute(
                body.into_use_case(termin_id)
                    .change_context(ExecutionError)?,
            )
            .await?;
        Ok(schema_seminar_termin_from_domain(termin))
    }

    pub async fn seminar_buchung_stornieren(
        &self,
        termin_id: Uuid,
        buchung: Uuid,
    ) -> ResultReport<SeminarTermin, ExecutionError> {
        let termin = self
            .app
            .execute(SeminarBuchungStornieren {
                termin_id: SeminarTerminId(termin_id),
                buchung_id: buchung_id(buchung),
            })
            .await?;
        Ok(schema_seminar_termin_from_domain(termin))
    }

    pub async fn seminar_termin_absagen(
        &self,
        termin_id: Uuid,
        body: SeminarTerminAbsage,
    ) -> ResultReport<SeminarTermin, ExecutionError> {
        let termin = self.app.execute(body.into_use_case(termin_id)).await?;
        Ok(schema_seminar_termin_from_domain(termin))
    }

    pub async fn seminar_termin_abgehalten(
        &self,
        termin_id: Uuid,
    ) -> ResultReport<SeminarTermin, ExecutionError> {
        let termin = self.app.execute(abgehalten_use_case(termin_id)).await?;
        Ok(schema_seminar_termin_from_domain(termin))
    }

    pub async fn seminar_umsatz_vorschau(
        &self,
        termin_id: Uuid,
    ) -> ResultReport<SeminarUmsatzVorschau, ExecutionError> {
        let umsatz = self
            .app
            .execute(yams_core::service::SeminarUmsatzVorschau {
                termin_id: SeminarTerminId(termin_id),
            })
            .await?;
        Ok(schema_umsatz_from_domain(umsatz))
    }

    pub async fn seminar_umsatz_prognose(
        &self,
        stichtag: chrono::NaiveDate,
    ) -> ResultReport<SeminarUmsatzPrognose, ExecutionError> {
        let prognose = self
            .app
            .execute(SeminarUmsatzPrognoseBisDatum { stichtag })
            .await?;
        Ok(schema_prognose_from_domain(prognose))
    }
}
