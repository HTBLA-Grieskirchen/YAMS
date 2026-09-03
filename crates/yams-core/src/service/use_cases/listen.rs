use async_trait::async_trait;
use error_stack::{Report, ResultExt};

use crate::{
    ResultReport,
    domain::{
        Behandlung, Haustier, Klient, Leistung, Produkt, Rechnung, Seminar, SeminarTermin,
    },
    service::{ExecutionContext, UseCase},
    uow::Versioned,
};

#[derive(thiserror::Error, Debug)]
pub enum AuflistenFehler {
    #[error("persistenzfehler")]
    Persistenz,
}

pub struct AufgelisteterKlient {
    pub klient: Klient,
    pub haustiere: Vec<Haustier>,
}

#[derive(Clone, Copy, Default)]
pub struct AlleKlientenAuflisten;

#[derive(Clone, Copy, Default)]
pub struct AlleHaustiereAuflisten;

#[derive(Clone, Copy, Default)]
pub struct AlleProdukteAuflisten;

#[derive(Clone, Copy, Default)]
pub struct AlleBehandlungenAuflisten;

#[derive(Clone, Copy, Default)]
pub struct AlleLeistungenAuflisten;

#[derive(Clone, Copy, Default)]
pub struct AlleRechnungenAuflisten;

#[derive(Clone, Copy, Default)]
pub struct AlleSeminareAuflisten;

#[derive(Clone, Copy, Default)]
pub struct AlleSeminarTermineAuflisten;

#[async_trait]
impl UseCase<Vec<AufgelisteterKlient>> for AlleKlientenAuflisten {
    type Error = AuflistenFehler;

    async fn perform(
        self,
        ctx: ExecutionContext<'_>,
    ) -> Result<Vec<AufgelisteterKlient>, Report<Self::Error>> {
        let uow = ctx
            .enter()
            .await
            .change_context(AuflistenFehler::Persistenz)?;

        let result = async {
            let klienten = uow
                .klienten()
                .find_all()
                .await
                .change_context(AuflistenFehler::Persistenz)?;

            let mut aufgelistet = Vec::with_capacity(klienten.len());
            for versioned in klienten {
                let klient_id = versioned.id().clone();
                let klient = versioned.into_data();
                let haustiere = uow
                    .haustiere()
                    .find_by_klient_id(klient_id)
                    .await
                    .change_context(AuflistenFehler::Persistenz)?
                    .into_iter()
                    .map(Versioned::into_data)
                    .collect();
                aufgelistet.push(AufgelisteterKlient { klient, haustiere });
            }
            Ok(aufgelistet)
        }
        .await;

        uow.finish(result, AuflistenFehler::Persistenz).await
    }
}

macro_rules! impl_einfaches_auflisten {
    ($use_case:ty, $repo:ident, $entity:ty) => {
        #[async_trait]
        impl UseCase<Vec<$entity>> for $use_case {
            type Error = AuflistenFehler;

            async fn perform(
                self,
                ctx: ExecutionContext<'_>,
            ) -> Result<Vec<$entity>, Report<Self::Error>> {
                let uow = ctx
                    .enter()
                    .await
                    .change_context(AuflistenFehler::Persistenz)?;

                let result = async {
                    let entities = uow
                        .$repo()
                        .find_all()
                        .await
                        .change_context(AuflistenFehler::Persistenz)?
                        .into_iter()
                        .map(Versioned::into_data)
                        .collect();
                    Ok(entities)
                }
                .await;

                uow.finish(result, AuflistenFehler::Persistenz).await
            }
        }
    };
}

impl_einfaches_auflisten!(AlleHaustiereAuflisten, haustiere, Haustier);
impl_einfaches_auflisten!(AlleProdukteAuflisten, produkte, Produkt);
impl_einfaches_auflisten!(AlleBehandlungenAuflisten, behandlungen, Behandlung);
impl_einfaches_auflisten!(AlleLeistungenAuflisten, leistungen, Leistung);
impl_einfaches_auflisten!(AlleRechnungenAuflisten, rechnungen, Rechnung);
impl_einfaches_auflisten!(AlleSeminareAuflisten, seminare, Seminar);
impl_einfaches_auflisten!(AlleSeminarTermineAuflisten, seminar_termine, SeminarTermin);
