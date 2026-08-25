use error_stack::Report;
use http::StatusCode;
use poem_openapi::{
    OpenApi, OpenApiService, ServerObject,
    param::{Path, Query},
    payload::{Json, PlainText},
    types::ToJSON,
};
use uuid::Uuid;
use yams_core::{App, ThreadSafeError};

use crate::{
    api::YamsAppApi,
    errors::{InternalServerError, StructuredError},
    requests::{
        BehandlungErstellung, HaustierErstellung, KlientErstellung,
        LeistungAusBehandlungErstellung, LeistungAusProduktErstellung, LeistungManuelleErstellung,
        ProduktErstellung, SeminarBuchungErstellung, SeminarErstellung, SeminarTerminAbsage,
        SeminarTerminAktualisierung, SeminarTerminErstellung, TagesabschlussErstellung,
    },
    schema::{
        Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung, Seminar, SeminarTerminDto,
        SeminarUmsatzPrognose, SeminarUmsatzVorschau,
    },
};

pub struct YamsApiSpec {
    app_api: YamsAppApi,
}

impl YamsApiSpec {
    pub fn new(app_api: YamsAppApi) -> Self {
        Self { app_api }
    }
}

impl From<YamsAppApi> for YamsApiSpec {
    fn from(app_api: YamsAppApi) -> Self {
        Self::new(app_api)
    }
}

impl From<App> for YamsApiSpec {
    fn from(app: App) -> Self {
        Self::new(YamsAppApi::new(app))
    }
}

#[derive(poem_openapi::ApiResponse)]
pub enum TypicalJsonResponse<T: ToJSON> {
    #[oai(status = 200)]
    Ok(Json<T>),
    #[oai(status_range = "4XX")]
    ClientError(StatusCode, Json<StructuredError>),
    #[oai(status = 500)]
    InternalError(PlainText<InternalServerError>),
}

impl<T: ToJSON, C: ThreadSafeError> From<Result<T, Report<C>>> for TypicalJsonResponse<T> {
    fn from(result: Result<T, Report<C>>) -> Self {
        match result {
            Ok(value) => TypicalJsonResponse::Ok(Json(value)),
            Err(error) => {
                let status = error
                    .request_value::<StatusCode>()
                    .next()
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                if status.is_server_error() {
                    return TypicalJsonResponse::InternalError(PlainText(InternalServerError));
                }
                TypicalJsonResponse::ClientError(status, Json(error.into()))
            }
        }
    }
}

#[OpenApi]
impl YamsApiSpec {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) -> Json<String> {
        Json("OK".to_string())
    }

    #[oai(path = "/klient", method = "post")]
    async fn klient_erstellen(&self, body: Json<KlientErstellung>) -> TypicalJsonResponse<Klient> {
        self.app_api.klient_erstellen(body.0).await.into()
    }

    #[oai(path = "/haustier", method = "post")]
    async fn haustier_erstellen(
        &self,
        body: Json<HaustierErstellung>,
    ) -> TypicalJsonResponse<Haustier> {
        self.app_api.haustier_erstellen(body.0).await.into()
    }

    #[oai(path = "/haustier", method = "get")]
    async fn alle_haustiere(&self) -> TypicalJsonResponse<Vec<Haustier>> {
        self.app_api.alle_haustiere().await.into()
    }

    #[oai(path = "/haustier/:id", method = "get")]
    async fn haustier_by_id(&self, id: Path<Uuid>) -> TypicalJsonResponse<Haustier> {
        self.app_api.haustier_by_id(id.0).await.into()
    }

    #[oai(path = "/produkt", method = "post")]
    async fn produkt_erstellen(
        &self,
        body: Json<ProduktErstellung>,
    ) -> TypicalJsonResponse<Produkt> {
        self.app_api.produkt_erstellen(body.0).await.into()
    }

    #[oai(path = "/behandlung", method = "post")]
    async fn behandlung_erstellen(
        &self,
        body: Json<BehandlungErstellung>,
    ) -> TypicalJsonResponse<Behandlung> {
        self.app_api.behandlung_erstellen(body.0).await.into()
    }

    #[oai(path = "/leistung/produkt", method = "post")]
    async fn leistung_aus_produkt_buchen(
        &self,
        body: Json<LeistungAusProduktErstellung>,
    ) -> TypicalJsonResponse<Leistung> {
        self.app_api
            .leistung_aus_produkt_buchen(body.0)
            .await
            .into()
    }

    #[oai(path = "/leistung/behandlung", method = "post")]
    async fn leistung_aus_behandlung_buchen(
        &self,
        body: Json<LeistungAusBehandlungErstellung>,
    ) -> TypicalJsonResponse<Leistung> {
        self.app_api
            .leistung_aus_behandlung_buchen(body.0)
            .await
            .into()
    }

    #[oai(path = "/leistung/manuell", method = "post")]
    async fn leistung_manuell_erfassen(
        &self,
        body: Json<LeistungManuelleErstellung>,
    ) -> TypicalJsonResponse<Leistung> {
        self.app_api.leistung_manuell_erfassen(body.0).await.into()
    }

    #[oai(path = "/tagesabschluss", method = "post")]
    async fn tagesabschluss_durchführen(
        &self,
        body: Json<TagesabschlussErstellung>,
    ) -> TypicalJsonResponse<Vec<Rechnung>> {
        self.app_api.tagesabschluss_durchführen(body.0).await.into()
    }

    #[oai(path = "/rechnung/:klient_id", method = "get")]
    async fn rechnungen_für_klient(
        &self,
        klient_id: Path<Uuid>,
    ) -> TypicalJsonResponse<Vec<Rechnung>> {
        self.app_api.rechnungen_für_klient(klient_id.0).await.into()
    }

    #[oai(path = "/seminar", method = "post")]
    async fn seminar_erstellen(
        &self,
        body: Json<SeminarErstellung>,
    ) -> TypicalJsonResponse<Seminar> {
        self.app_api.seminar_erstellen(body.0).await.into()
    }

    #[oai(path = "/seminar/:id", method = "get")]
    async fn seminar_by_id(&self, id: Path<Uuid>) -> TypicalJsonResponse<Seminar> {
        self.app_api.seminar_by_id(id.0).await.into()
    }

    #[oai(path = "/seminar-termin", method = "post")]
    async fn seminar_termin_planen(
        &self,
        body: Json<SeminarTerminErstellung>,
    ) -> TypicalJsonResponse<SeminarTerminDto> {
        self.app_api.seminar_termin_planen(body.0).await.into()
    }

    #[oai(path = "/seminar-termin/:id", method = "get")]
    async fn seminar_termin_by_id(
        &self,
        id: Path<Uuid>,
    ) -> TypicalJsonResponse<SeminarTerminDto> {
        self.app_api.seminar_termin_by_id(id.0).await.into()
    }

    #[oai(path = "/seminar-termin/:id", method = "put")]
    async fn seminar_termin_aktualisieren(
        &self,
        id: Path<Uuid>,
        body: Json<SeminarTerminAktualisierung>,
    ) -> TypicalJsonResponse<SeminarTerminDto> {
        self.app_api
            .seminar_termin_aktualisieren(id.0, body.0)
            .await
            .into()
    }

    #[oai(path = "/seminar-termin/:id/buchung", method = "post")]
    async fn seminar_buchung_anlegen(
        &self,
        id: Path<Uuid>,
        body: Json<SeminarBuchungErstellung>,
    ) -> TypicalJsonResponse<SeminarTerminDto> {
        self.app_api
            .seminar_buchung_anlegen(id.0, body.0)
            .await
            .into()
    }

    #[oai(path = "/seminar-termin/:id/buchung/:buchung_id/storno", method = "post")]
    async fn seminar_buchung_stornieren(
        &self,
        id: Path<Uuid>,
        buchung_id: Path<Uuid>,
    ) -> TypicalJsonResponse<SeminarTerminDto> {
        self.app_api
            .seminar_buchung_stornieren(id.0, buchung_id.0)
            .await
            .into()
    }

    #[oai(path = "/seminar-termin/:id/absagen", method = "post")]
    async fn seminar_termin_absagen(
        &self,
        id: Path<Uuid>,
        body: Json<SeminarTerminAbsage>,
    ) -> TypicalJsonResponse<SeminarTerminDto> {
        self.app_api.seminar_termin_absagen(id.0, body.0).await.into()
    }

    #[oai(path = "/seminar-termin/:id/abgehalten", method = "post")]
    async fn seminar_termin_abgehalten(
        &self,
        id: Path<Uuid>,
    ) -> TypicalJsonResponse<SeminarTerminDto> {
        self.app_api.seminar_termin_abgehalten(id.0).await.into()
    }

    #[oai(path = "/seminar-termin/:id/umsatz", method = "get")]
    async fn seminar_umsatz_vorschau(
        &self,
        id: Path<Uuid>,
    ) -> TypicalJsonResponse<SeminarUmsatzVorschau> {
        self.app_api.seminar_umsatz_vorschau(id.0).await.into()
    }

    #[oai(path = "/seminar-prognose", method = "get")]
    async fn seminar_umsatz_prognose(
        &self,
        stichtag: Query<chrono::NaiveDate>,
    ) -> TypicalJsonResponse<SeminarUmsatzPrognose> {
        self.app_api.seminar_umsatz_prognose(stichtag.0).await.into()
    }
}

pub fn openapi_service(
    app: App,
    server_urls: impl IntoIterator<Item = impl Into<ServerObject>>,
) -> OpenApiService<YamsApiSpec, ()> {
    let mut service = OpenApiService::new(
        YamsApiSpec::from(app),
        "YAMS API",
        env!("CARGO_PKG_VERSION"),
    )
    .description(env!("CARGO_PKG_DESCRIPTION"));
    for server_url in server_urls {
        service = service.server(server_url);
    }
    service
}
