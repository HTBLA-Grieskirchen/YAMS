use async_trait::async_trait;
use chrono::NaiveDate;
use error_stack::{IntoReport, Report, ResultExt};
use tracing::info;

use crate::{
    ResultReport,
    domain::{Haustier, KlientId, haustier::NeuesHaustier},
    service::{ExecutionContext, UseCase},
};

#[derive(Clone)]
pub struct HaustierErstellen {
    pub klient_id: KlientId,
    pub name: String,
    pub geburtstag: NaiveDate,
    pub tierart: String,
    pub beschreibung: String,
}

#[derive(thiserror::Error, Debug)]
pub enum HaustierErstellenFehler {
    #[error("persistenzfehler")]
    Persistenz,
    #[error("klient mit id `{0:?}` nicht gefunden")]
    KlientNichtGefunden(KlientId),
}

#[async_trait]
impl UseCase<Haustier> for HaustierErstellen {
    type Error = HaustierErstellenFehler;

    async fn perform(self, ctx: ExecutionContext<'_>) -> Result<Haustier, Report<Self::Error>> {
        let uow = ctx
            .enter()
            .await
            .change_context(HaustierErstellenFehler::Persistenz)?;

        let result = async {
            uow.klienten()
                .find_by_id(self.klient_id.clone())
                .await
                .change_context(HaustierErstellenFehler::KlientNichtGefunden(
                    self.klient_id.clone(),
                ))?;

            let haustier = uow
                .haustiere()
                .create(NeuesHaustier::neu(
                    self.klient_id,
                    self.name,
                    self.geburtstag,
                    self.tierart,
                    self.beschreibung,
                ))
                .await
                .change_context(HaustierErstellenFehler::Persistenz)?;

            Ok(haustier.into_data())
        }
        .await;

        let haustier = uow
            .finish(result, HaustierErstellenFehler::Persistenz)
            .await?;
        info!(
            id = ?haustier.id(),
            klient_id = ?haustier.klient_id(),
            "haustier angelegt"
        );
        Ok(haustier)
    }
}

#[derive(Clone)]
pub struct VieleHaustiereErstellen {
    pub haustiere: Vec<HaustierErstellen>,
}

#[derive(thiserror::Error, Debug)]
#[error("{failures} von {target} haustieren konnten nicht angelegt werden")]
pub struct VieleHaustiereErstellenFehler {
    failures: usize,
    target: usize,
}

#[async_trait]
impl UseCase<Vec<Haustier>> for VieleHaustiereErstellen {
    type Error = Report<[HaustierErstellenFehler]>;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> ResultReport<Vec<Haustier>, <Self::Error as IntoReport>::Context> {
        let uow = ctx
            .enter()
            .await
            .change_context(HaustierErstellenFehler::Persistenz)
            .map_err(|e| e.expand())?;

        let mut errors = Option::<Report<[HaustierErstellenFehler]>>::None;
        let mut haustiere = Vec::with_capacity(self.haustiere.len());
        for fut in self.haustiere.into_iter().map(|h| h.perform(ctx.sub(&uow))) {
            match fut.await {
                Ok(haustier) => haustiere.push(haustier),
                Err(e) => match &mut errors {
                    Some(errors) => errors.push(e),
                    None => errors = Some(e.expand()),
                },
            }
        }
        if let Some(errors) = errors {
            let _ = uow.rollback().await;
            return Err(errors);
        }

        uow.commit()
            .await
            .change_context(HaustierErstellenFehler::Persistenz)
            .map_err(|e| e.expand())?;

        info!(anzahl = haustiere.len(), "haustiere angelegt");
        Ok(haustiere)
    }
}
