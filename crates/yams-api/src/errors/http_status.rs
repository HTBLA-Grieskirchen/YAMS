use http::StatusCode;
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

pub trait HttpStatusMapping {
    fn http_status(&self) -> Option<StatusCode> {
        None
    }
}

impl HttpStatusMapping for ValidationError {
    fn http_status(&self) -> Option<StatusCode> {
        Some(StatusCode::BAD_REQUEST)
    }
}

impl HttpStatusMapping for RepositoryError {
    fn http_status(&self) -> Option<StatusCode> {
        match self {
            RepositoryError::NotFound => Some(StatusCode::NOT_FOUND),
            _ => None,
        }
    }
}

impl HttpStatusMapping for ObjectStoreError {}

macro_rules! default_http_status {
    ($($ty:ty),+ $(,)?) => {
        $(impl HttpStatusMapping for $ty {})+
    };
}

default_http_status! {
    KlientErstellenFehler,
    HaustierErstellenFehler,
    ProduktErstellenFehler,
    BehandlungErstellenFehler,
    LeistungAusProduktBuchenFehler,
    LeistungAusBehandlungBuchenFehler,
    LeistungManuellErfassenFehler,
    TagesabschlussDurchführenFehler,
    SeminarErstellenFehler,
    SeminarTerminPlanenFehler,
    SeminarTerminAktualisierenFehler,
    SeminarBuchungAnlegenFehler,
    SeminarBuchungStornierenFehler,
    SeminarTerminAbsagenFehler,
    SeminarTerminAlsAbgehaltenMarkierenFehler,
    SeminarUmsatzVorschauFehler,
    SeminarUmsatzPrognoseBisDatumFehler,
}
