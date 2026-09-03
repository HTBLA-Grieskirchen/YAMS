use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{IntoReport, Report, ResultExt};

use crate::{
    application::uow::Versioned,
    domain::{Adresse, EmailAdresse, Klient, Mobilnummer, klient::NeuerKlient},
    service::{ExecutionContext, UseCase},
};

#[derive(Clone)]
pub struct KlientErstellen {
    pub vorname: String,
    pub nachname: String,
    pub geburtstag: NaiveDate,
    pub email: EmailAdresse,
    pub mobilnummer: Mobilnummer,
    pub kundennummer: u64,
    pub einwilligung: bool,
    pub adresse: Adresse,
}

#[derive(thiserror::Error, Debug)]
pub enum KlientErstellenFehler {
    #[error("fehler beim anlegen des klienten")]
    Erstellung,
}

#[async_trait]
impl UseCase<Klient> for KlientErstellen {
    type Error = Report<KlientErstellenFehler>;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Klient, Self::Error> {
        let uow = ctx
            .enter()
            .await
            .change_context(KlientErstellenFehler::Erstellung)?;

        let result = uow
            .klienten()
            .create(NeuerKlient::neu(
                self.vorname,
                self.nachname,
                self.geburtstag,
                self.email,
                self.mobilnummer,
                self.kundennummer,
                self.einwilligung,
                self.adresse,
            ))
            .await
            .map(Versioned::into_data)
            .map_err(IntoReport::into_report)
            .change_context(KlientErstellenFehler::Erstellung);

        uow.finish(result, KlientErstellenFehler::Erstellung)
            .await
    }
}
