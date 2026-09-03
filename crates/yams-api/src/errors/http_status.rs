use error_stack::Report;
use http::StatusCode;
use yams_core::ThreadSafeError;
use yams_core::domain::{
    SeminarTerminFehler, behandlung::BehandlungFehler, haustier::HaustierFehler,
    klient::KlientFehler, leistung::LeistungFehler, produkt::ProduktFehler,
    rechnung::RechnungFehler, seminar::SeminarFehler, zeitraum::ZeitraumFehler,
};
use yams_core::ports::{ObjectStoreError, RepositoryError};
use yams_core::service::{
    BehandlungErstellenFehler, HaustierErstellenFehler, KlientErstellenFehler,
    LeistungAusBehandlungBuchenFehler, LeistungAusProduktBuchenFehler,
    LeistungManuellErfassenFehler, ProduktErstellenFehler, SeminarBuchungAnlegenFehler,
    SeminarBuchungStornierenFehler, SeminarErstellenFehler, SeminarTerminAbsagenFehler,
    SeminarTerminAktualisierenFehler, SeminarTerminAlsAbgehaltenMarkierenFehler,
    SeminarTerminPlanenFehler, SeminarUmsatzPrognoseBisDatumFehler, SeminarUmsatzVorschauFehler,
    TagesabschlussDurchführenFehler,
};

use super::ValidationError;

/// Maps a typed error context to an HTTP status.
///
/// `None` means "not this layer" — [`status_from_report`] then walks nested
/// contexts (domain, repository) before defaulting to 500.
pub trait HttpStatusMapping {
    fn http_status(&self) -> Option<StatusCode> {
        None
    }
}

pub fn status_from_report<C: ThreadSafeError + HttpStatusMapping>(error: &Report<C>) -> StatusCode {
    error
        .request_value::<StatusCode>()
        .next()
        .or_else(|| error.downcast_ref::<StatusCode>().copied())
        .or_else(|| error.current_context().http_status())
        .or_else(|| nested_http_status(error))
        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
}

fn mapped<C: ThreadSafeError, T: ThreadSafeError + HttpStatusMapping>(
    error: &Report<C>,
) -> Option<StatusCode> {
    error
        .downcast_ref::<T>()
        .and_then(HttpStatusMapping::http_status)
}

fn nested_http_status<C: ThreadSafeError>(error: &Report<C>) -> Option<StatusCode> {
    mapped::<C, SeminarTerminFehler>(error)
        .or_else(|| mapped::<C, RechnungFehler>(error))
        .or_else(|| mapped::<C, KlientFehler>(error))
        .or_else(|| mapped::<C, HaustierFehler>(error))
        .or_else(|| mapped::<C, ProduktFehler>(error))
        .or_else(|| mapped::<C, BehandlungFehler>(error))
        .or_else(|| mapped::<C, SeminarFehler>(error))
        .or_else(|| mapped::<C, LeistungFehler>(error))
        .or_else(|| mapped::<C, ZeitraumFehler>(error))
        .or_else(|| mapped::<C, SeminarUmsatzVorschauFehler>(error))
        .or_else(|| mapped::<C, HaustierErstellenFehler>(error))
        .or_else(|| mapped::<C, LeistungAusProduktBuchenFehler>(error))
        .or_else(|| mapped::<C, LeistungAusBehandlungBuchenFehler>(error))
        .or_else(|| mapped::<C, TagesabschlussDurchführenFehler>(error))
        .or_else(|| mapped::<C, SeminarTerminPlanenFehler>(error))
        .or_else(|| mapped::<C, SeminarTerminAktualisierenFehler>(error))
        .or_else(|| mapped::<C, SeminarBuchungAnlegenFehler>(error))
        .or_else(|| mapped::<C, SeminarBuchungStornierenFehler>(error))
        .or_else(|| mapped::<C, SeminarTerminAbsagenFehler>(error))
        .or_else(|| mapped::<C, SeminarTerminAlsAbgehaltenMarkierenFehler>(error))
        .or_else(|| mapped::<C, RepositoryError>(error))
        .or_else(|| mapped::<C, ObjectStoreError>(error))
        .or_else(|| mapped::<C, ValidationError>(error))
}

impl HttpStatusMapping for ValidationError {
    fn http_status(&self) -> Option<StatusCode> {
        Some(StatusCode::BAD_REQUEST)
    }
}

impl HttpStatusMapping for RepositoryError {
    fn http_status(&self) -> Option<StatusCode> {
        Some(match self {
            RepositoryError::NotFound => StatusCode::NOT_FOUND,
            RepositoryError::VersionMismatch { .. } | RepositoryError::Conflict => {
                StatusCode::CONFLICT
            }
            RepositoryError::Permission => StatusCode::FORBIDDEN,
            RepositoryError::Connection => StatusCode::SERVICE_UNAVAILABLE,
            RepositoryError::OperationFailed
            | RepositoryError::Storage
            | RepositoryError::Data
            | RepositoryError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        })
    }
}

impl HttpStatusMapping for ObjectStoreError {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            ObjectStoreError::Operation => Some(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl HttpStatusMapping for SeminarTerminFehler {
    fn http_status(&self) -> Option<StatusCode> {
        Some(match self {
            SeminarTerminFehler::BuchungNichtGefunden => StatusCode::NOT_FOUND,
            SeminarTerminFehler::KapazitätErreicht
            | SeminarTerminFehler::KlientBereitsGebucht
            | SeminarTerminFehler::BuchungBereitsStorniert
            | SeminarTerminFehler::KapazitätUnterBestätigten => StatusCode::CONFLICT,
            SeminarTerminFehler::LeistungenUnvollständig => StatusCode::INTERNAL_SERVER_ERROR,
        })
    }
}

impl HttpStatusMapping for RechnungFehler {
    fn http_status(&self) -> Option<StatusCode> {
        Some(match self {
            RechnungFehler::KeineLeistungen => StatusCode::UNPROCESSABLE_ENTITY,
            RechnungFehler::KlientUnstimmig => StatusCode::CONFLICT,
        })
    }
}

macro_rules! client_validation {
    ($($ty:ty),+ $(,)?) => {
        $(impl HttpStatusMapping for $ty {
            fn http_status(&self) -> Option<StatusCode> {
                Some(StatusCode::BAD_REQUEST)
            }
        })+
    };
}

client_validation! {
    KlientFehler,
    HaustierFehler,
    ProduktFehler,
    BehandlungFehler,
    SeminarFehler,
    LeistungFehler,
    ZeitraumFehler,
}

impl HttpStatusMapping for KlientErstellenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            KlientErstellenFehler::Erstellung => None,
        }
    }
}

impl HttpStatusMapping for HaustierErstellenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            HaustierErstellenFehler::Persistenz => None,
            HaustierErstellenFehler::KlientNichtGefunden(_) => Some(StatusCode::NOT_FOUND),
        }
    }
}

impl HttpStatusMapping for ProduktErstellenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            ProduktErstellenFehler::Erstellung => None,
        }
    }
}

impl HttpStatusMapping for BehandlungErstellenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            BehandlungErstellenFehler::Erstellung => None,
        }
    }
}

impl HttpStatusMapping for LeistungAusProduktBuchenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            LeistungAusProduktBuchenFehler::Persistenz => None,
            LeistungAusProduktBuchenFehler::ProduktNichtGefunden => Some(StatusCode::NOT_FOUND),
        }
    }
}

impl HttpStatusMapping for LeistungAusBehandlungBuchenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            LeistungAusBehandlungBuchenFehler::Persistenz => None,
            LeistungAusBehandlungBuchenFehler::BehandlungNichtGefunden => {
                Some(StatusCode::NOT_FOUND)
            }
        }
    }
}

impl HttpStatusMapping for LeistungManuellErfassenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            LeistungManuellErfassenFehler::Persistenz => None,
        }
    }
}

impl HttpStatusMapping for TagesabschlussDurchführenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            TagesabschlussDurchführenFehler::Persistenz
            | TagesabschlussDurchführenFehler::Rechnung => None,
            TagesabschlussDurchführenFehler::KlientNichtGefunden => Some(StatusCode::NOT_FOUND),
            TagesabschlussDurchführenFehler::Pdf | TagesabschlussDurchführenFehler::Speicher => {
                Some(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl HttpStatusMapping for SeminarErstellenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SeminarErstellenFehler::Erstellung => None,
        }
    }
}

impl HttpStatusMapping for SeminarTerminPlanenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SeminarTerminPlanenFehler::Persistenz => None,
            SeminarTerminPlanenFehler::SeminarNichtGefunden => Some(StatusCode::NOT_FOUND),
        }
    }
}

impl HttpStatusMapping for SeminarTerminAktualisierenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SeminarTerminAktualisierenFehler::Persistenz
            | SeminarTerminAktualisierenFehler::Invariante => None,
            SeminarTerminAktualisierenFehler::TerminNichtGefunden => Some(StatusCode::NOT_FOUND),
            SeminarTerminAktualisierenFehler::NichtGeplant => Some(StatusCode::CONFLICT),
        }
    }
}

impl HttpStatusMapping for SeminarBuchungAnlegenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SeminarBuchungAnlegenFehler::Persistenz | SeminarBuchungAnlegenFehler::Invariante => {
                None
            }
            SeminarBuchungAnlegenFehler::TerminNichtGefunden
            | SeminarBuchungAnlegenFehler::KlientNichtGefunden => Some(StatusCode::NOT_FOUND),
            SeminarBuchungAnlegenFehler::NichtGeplant => Some(StatusCode::CONFLICT),
        }
    }
}

impl HttpStatusMapping for SeminarBuchungStornierenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SeminarBuchungStornierenFehler::Persistenz
            | SeminarBuchungStornierenFehler::Invariante => None,
            SeminarBuchungStornierenFehler::TerminNichtGefunden => Some(StatusCode::NOT_FOUND),
            SeminarBuchungStornierenFehler::NichtGeplant => Some(StatusCode::CONFLICT),
        }
    }
}

impl HttpStatusMapping for SeminarTerminAbsagenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SeminarTerminAbsagenFehler::Persistenz => None,
            SeminarTerminAbsagenFehler::TerminNichtGefunden => Some(StatusCode::NOT_FOUND),
            SeminarTerminAbsagenFehler::NichtGeplant => Some(StatusCode::CONFLICT),
        }
    }
}

impl HttpStatusMapping for SeminarTerminAlsAbgehaltenMarkierenFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SeminarTerminAlsAbgehaltenMarkierenFehler::Persistenz
            | SeminarTerminAlsAbgehaltenMarkierenFehler::Invariante => None,
            SeminarTerminAlsAbgehaltenMarkierenFehler::TerminNichtGefunden
            | SeminarTerminAlsAbgehaltenMarkierenFehler::SeminarNichtGefunden
            | SeminarTerminAlsAbgehaltenMarkierenFehler::KlientNichtGefunden => {
                Some(StatusCode::NOT_FOUND)
            }
            SeminarTerminAlsAbgehaltenMarkierenFehler::NichtGeplant => Some(StatusCode::CONFLICT),
            SeminarTerminAlsAbgehaltenMarkierenFehler::Pdf
            | SeminarTerminAlsAbgehaltenMarkierenFehler::Speicher => {
                Some(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl HttpStatusMapping for SeminarUmsatzVorschauFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SeminarUmsatzVorschauFehler::Persistenz => None,
            SeminarUmsatzVorschauFehler::TerminNichtGefunden
            | SeminarUmsatzVorschauFehler::SeminarNichtGefunden => Some(StatusCode::NOT_FOUND),
            SeminarUmsatzVorschauFehler::Abgesagt => Some(StatusCode::CONFLICT),
        }
    }
}

impl HttpStatusMapping for SeminarUmsatzPrognoseBisDatumFehler {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            SeminarUmsatzPrognoseBisDatumFehler::Persistenz => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;
    use yams_core::domain::KlientId;

    use super::*;

    fn status<C: ThreadSafeError + HttpStatusMapping>(ctx: C) -> StatusCode {
        status_from_report(&Report::new(ctx))
    }

    fn wrapped<
        Inner: ThreadSafeError + HttpStatusMapping,
        Outer: ThreadSafeError + HttpStatusMapping,
    >(
        inner: Inner,
        outer: Outer,
    ) -> StatusCode {
        status_from_report(&Report::new(inner).change_context(outer))
    }

    #[test]
    fn attached_status_wins() {
        let error =
            Report::new(HaustierErstellenFehler::Persistenz).attach_opaque(StatusCode::NOT_FOUND);
        assert_eq!(status_from_report(&error), StatusCode::NOT_FOUND);
    }

    #[test]
    fn validation_is_bad_request() {
        assert_eq!(status(ValidationError), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn repository_variants() {
        assert_eq!(status(RepositoryError::NotFound), StatusCode::NOT_FOUND);
        assert_eq!(
            status(RepositoryError::VersionMismatch {
                expected: 1,
                actual: Some(2)
            }),
            StatusCode::CONFLICT
        );
        assert_eq!(status(RepositoryError::Conflict), StatusCode::CONFLICT);
        assert_eq!(status(RepositoryError::Permission), StatusCode::FORBIDDEN);
        assert_eq!(
            status(RepositoryError::Connection),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            status(RepositoryError::OperationFailed),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn use_case_not_found_and_conflict() {
        assert_eq!(
            status(HaustierErstellenFehler::KlientNichtGefunden(KlientId(
                Uuid::nil()
            ))),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            status(LeistungAusProduktBuchenFehler::ProduktNichtGefunden),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            status(LeistungAusBehandlungBuchenFehler::BehandlungNichtGefunden),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            status(TagesabschlussDurchführenFehler::KlientNichtGefunden),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            status(SeminarTerminPlanenFehler::SeminarNichtGefunden),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            status(SeminarTerminAktualisierenFehler::TerminNichtGefunden),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            status(SeminarTerminAktualisierenFehler::NichtGeplant),
            StatusCode::CONFLICT
        );
        assert_eq!(
            status(SeminarBuchungAnlegenFehler::KlientNichtGefunden),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            status(SeminarBuchungAnlegenFehler::NichtGeplant),
            StatusCode::CONFLICT
        );
        assert_eq!(
            status(SeminarBuchungStornierenFehler::NichtGeplant),
            StatusCode::CONFLICT
        );
        assert_eq!(
            status(SeminarTerminAbsagenFehler::NichtGeplant),
            StatusCode::CONFLICT
        );
        assert_eq!(
            status(SeminarTerminAlsAbgehaltenMarkierenFehler::NichtGeplant),
            StatusCode::CONFLICT
        );
        assert_eq!(
            status(SeminarUmsatzVorschauFehler::Abgesagt),
            StatusCode::CONFLICT
        );
        assert_eq!(
            status(SeminarUmsatzVorschauFehler::TerminNichtGefunden),
            StatusCode::NOT_FOUND
        );
    }

    #[test]
    fn persistenz_falls_through_to_repository() {
        assert_eq!(
            wrapped(
                RepositoryError::NotFound,
                HaustierErstellenFehler::Persistenz
            ),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            wrapped(
                RepositoryError::VersionMismatch {
                    expected: 1,
                    actual: None
                },
                KlientErstellenFehler::Erstellung
            ),
            StatusCode::CONFLICT
        );
        assert_eq!(
            wrapped(
                RepositoryError::Connection,
                LeistungManuellErfassenFehler::Persistenz
            ),
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            status(KlientErstellenFehler::Erstellung),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status(SeminarUmsatzPrognoseBisDatumFehler::Persistenz),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }

    #[test]
    fn invariante_falls_through_to_domain() {
        assert_eq!(
            wrapped(
                SeminarTerminFehler::KapazitätErreicht,
                SeminarBuchungAnlegenFehler::Invariante
            ),
            StatusCode::CONFLICT
        );
        assert_eq!(
            wrapped(
                SeminarTerminFehler::BuchungNichtGefunden,
                SeminarBuchungStornierenFehler::Invariante
            ),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            wrapped(
                SeminarTerminFehler::LeistungenUnvollständig,
                SeminarTerminAlsAbgehaltenMarkierenFehler::Invariante
            ),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            wrapped(
                RechnungFehler::KeineLeistungen,
                TagesabschlussDurchführenFehler::Rechnung
            ),
            StatusCode::UNPROCESSABLE_ENTITY
        );
    }

    #[test]
    fn prognose_preserves_vorschau_status() {
        assert_eq!(
            wrapped(
                SeminarUmsatzVorschauFehler::TerminNichtGefunden,
                SeminarUmsatzPrognoseBisDatumFehler::Persistenz
            ),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            wrapped(
                SeminarUmsatzVorschauFehler::Abgesagt,
                SeminarUmsatzPrognoseBisDatumFehler::Persistenz
            ),
            StatusCode::CONFLICT
        );
    }

    #[test]
    fn domain_construction_is_bad_request() {
        assert_eq!(status(KlientFehler::NameLeer), StatusCode::BAD_REQUEST);
        assert_eq!(status(SeminarFehler::TitelLeer), StatusCode::BAD_REQUEST);
        assert_eq!(
            wrapped(KlientFehler::NameLeer, KlientErstellenFehler::Erstellung),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn infrastructure_failures_are_500() {
        assert_eq!(
            status(TagesabschlussDurchführenFehler::Pdf),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status(SeminarTerminAlsAbgehaltenMarkierenFehler::Speicher),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            status(ObjectStoreError::Operation),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
