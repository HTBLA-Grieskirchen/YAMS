//! Repository tracing wrappers. Each wrapper holds a pinned pointer to the parent
//! [`UnitOfWorkImpl`] and delegates through the matching accessor on every call.

use std::pin::Pin;

use async_trait::async_trait;
use chrono::NaiveDate;
use tracing::instrument;

use crate::application::uow::{UnitOfWorkImpl, Versioned};
use crate::domain::{
    Behandlung, BehandlungId, Haustier, HaustierId, Klient, KlientId, Leistung, LeistungId,
    LeistungOffen, Produkt, ProduktId, Rechnung, RechnungOffen, Seminar, SeminarId, SeminarTermin,
    SeminarTerminGeplant, SeminarTerminId, behandlung::NeueBehandlung, haustier::NeuesHaustier,
    klient::NeuerKlient, leistung::NeueLeistung, produkt::NeuesProdukt, seminar::NeuesSeminar,
    seminar_termin::NeuerSeminarTermin,
};
use crate::ports::{
    BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository,
    ProduktRepository, RechnungRepository, RepositoryResult, SeminarRepository,
    SeminarTerminRepository,
};

macro_rules! impl_instrumented_repo {
    (
        $wrapper:ident,
        $trait:ident,
        $accessor:ident,
        methods: [
            $(#[$meta:meta] fn $name:ident(&self $(, $arg:ident : $arg_ty:ty)*) -> $ret:ty;)*
        ]
    ) => {
        pub(crate) struct $wrapper {
            uow: Option<*const dyn UnitOfWorkImpl>,
        }

        impl $wrapper {
            pub(crate) fn new(uow: *const dyn UnitOfWorkImpl) -> Self {
                Self { uow: Some(uow) }
            }

            pub(crate) fn disabled() -> Self {
                Self { uow: None }
            }

            fn uow(&self) -> &dyn UnitOfWorkImpl {
                let uow = self
                    .uow
                    .expect("instrumented repository without unit of work");
                unsafe { &*uow }
            }
        }

        // SAFETY: `uow` points into a pinned `RepoStorage`; the address is stable until
        // the storage is consumed on commit/rollback.
        unsafe impl Send for $wrapper {}
        unsafe impl Sync for $wrapper {}

        #[async_trait]
        impl $trait for $wrapper {
            $(#[$meta]
            async fn $name(&self $(, $arg : $arg_ty)*) -> $ret {
                self.uow().$accessor().$name($($arg),*).await
            })*
        }
    };
}

impl_instrumented_repo!(
    InstrumentedKlientRepository,
    KlientRepository,
    klienten,
    methods: [
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: KlientId) -> RepositoryResult<Versioned<Klient>>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn find_all(&self) -> RepositoryResult<Vec<Versioned<Klient>>>;
        #[instrument(skip(self, klient), level = "debug", err(Debug))]
        fn create(&self, klient: NeuerKlient) -> RepositoryResult<Versioned<Klient>>;
        #[instrument(skip(self, klient), level = "debug", err(Debug))]
        fn update(&self, klient: &mut Versioned<Klient>) -> RepositoryResult<()>;
        #[instrument(skip(self, klient), fields(id = ?klient.id()), level = "debug", err(Debug))]
        fn delete(&self, klient: Versioned<Klient>) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedHaustierRepository,
    HaustierRepository,
    haustiere,
    methods: [
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: HaustierId) -> RepositoryResult<Versioned<Haustier>>;
        #[instrument(skip(self, klient_id), fields(klient_id = ?klient_id), level = "trace", err(Debug))]
        fn find_by_klient_id(&self, klient_id: KlientId) -> RepositoryResult<Vec<Versioned<Haustier>>>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn find_all(&self) -> RepositoryResult<Vec<Versioned<Haustier>>>;
        #[instrument(skip(self, haustier), level = "debug", err(Debug))]
        fn create(&self, haustier: NeuesHaustier) -> RepositoryResult<Versioned<Haustier>>;
        #[instrument(skip(self, haustier), level = "debug", err(Debug))]
        fn update(&self, haustier: &mut Versioned<Haustier>) -> RepositoryResult<()>;
        #[instrument(skip(self, haustier), fields(id = ?haustier.id()), level = "debug", err(Debug))]
        fn delete(&self, haustier: Versioned<Haustier>) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedProduktRepository,
    ProduktRepository,
    produkte,
    methods: [
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: ProduktId) -> RepositoryResult<Versioned<Produkt>>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn find_all(&self) -> RepositoryResult<Vec<Versioned<Produkt>>>;
        #[instrument(skip(self, produkt), level = "debug", err(Debug))]
        fn create(&self, produkt: NeuesProdukt) -> RepositoryResult<Versioned<Produkt>>;
    ]
);

impl_instrumented_repo!(
    InstrumentedBehandlungRepository,
    BehandlungRepository,
    behandlungen,
    methods: [
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: BehandlungId) -> RepositoryResult<Versioned<Behandlung>>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn find_all(&self) -> RepositoryResult<Vec<Versioned<Behandlung>>>;
        #[instrument(skip(self, behandlung), level = "debug", err(Debug))]
        fn create(&self, behandlung: NeueBehandlung) -> RepositoryResult<Versioned<Behandlung>>;
    ]
);

impl_instrumented_repo!(
    InstrumentedLeistungRepository,
    LeistungRepository,
    leistungen,
    methods: [
        #[instrument(skip(self, leistung), level = "debug", err(Debug))]
        fn create(&self, leistung: NeueLeistung) -> RepositoryResult<Versioned<LeistungOffen>>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn find_all(&self) -> RepositoryResult<Vec<Versioned<Leistung>>>;
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: LeistungId) -> RepositoryResult<Versioned<Leistung>>;
        #[instrument(skip(self, datum), %datum, level = "trace", err(Debug))]
        fn find_offene_by_datum(&self, datum: NaiveDate) -> RepositoryResult<Vec<Versioned<LeistungOffen>>>;
        #[instrument(skip(self, leistung), level = "debug", err(Debug))]
        fn update(&self, leistung: &mut Versioned<Leistung>) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedRechnungRepository,
    RechnungRepository,
    rechnungen,
    methods: [
        #[instrument(skip(self, rechnung), level = "debug", err(Debug))]
        fn create(&self, rechnung: RechnungOffen) -> RepositoryResult<Versioned<RechnungOffen>>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn nächste_rechnungsnummer(&self) -> RepositoryResult<u64>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn find_all(&self) -> RepositoryResult<Vec<Versioned<Rechnung>>>;
        #[instrument(skip(self, klient_id), fields(klient_id = ?klient_id), level = "trace", err(Debug))]
        fn find_by_klient_id(&self, klient_id: KlientId) -> RepositoryResult<Vec<Versioned<Rechnung>>>;
    ]
);

impl_instrumented_repo!(
    InstrumentedSeminarRepository,
    SeminarRepository,
    seminare,
    methods: [
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: SeminarId) -> RepositoryResult<Versioned<Seminar>>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn find_all(&self) -> RepositoryResult<Vec<Versioned<Seminar>>>;
        #[instrument(skip(self, seminar), level = "debug", err(Debug))]
        fn create(&self, seminar: NeuesSeminar) -> RepositoryResult<Versioned<Seminar>>;
        #[instrument(skip(self, seminar), level = "debug", err(Debug))]
        fn update(&self, seminar: &mut Versioned<Seminar>) -> RepositoryResult<()>;
    ]
);

impl_instrumented_repo!(
    InstrumentedSeminarTerminRepository,
    SeminarTerminRepository,
    seminar_termine,
    methods: [
        #[instrument(skip(self, id), fields(id = ?id), level = "trace", err(Debug))]
        fn find_by_id(&self, id: SeminarTerminId) -> RepositoryResult<Versioned<SeminarTermin>>;
        #[instrument(skip(self), level = "trace", err(Debug))]
        fn find_all(&self) -> RepositoryResult<Vec<Versioned<SeminarTermin>>>;
        #[instrument(skip(self, seminar_id), fields(seminar_id = ?seminar_id), level = "trace", err(Debug))]
        fn find_by_seminar_id(&self, seminar_id: SeminarId) -> RepositoryResult<Vec<Versioned<SeminarTermin>>>;
        #[instrument(skip(self, stichtag), %stichtag, level = "trace", err(Debug))]
        fn find_nicht_vollständig_abgerechnet_bis(&self, stichtag: NaiveDate) -> RepositoryResult<Vec<Versioned<SeminarTermin>>>;
        #[instrument(skip(self, termin), level = "debug", err(Debug))]
        fn create(&self, termin: NeuerSeminarTermin) -> RepositoryResult<Versioned<SeminarTerminGeplant>>;
        #[instrument(skip(self, termin), level = "debug", err(Debug))]
        fn update(&self, termin: &mut Versioned<SeminarTermin>) -> RepositoryResult<()>;
    ]
);

/// Pinned bundle: instrumented repos hold pointers into `uow` (field order keeps drop safe).
pub(crate) struct RepoStorage {
    pub(crate) klienten: InstrumentedKlientRepository,
    pub(crate) haustiere: InstrumentedHaustierRepository,
    pub(crate) produkte: InstrumentedProduktRepository,
    pub(crate) behandlungen: InstrumentedBehandlungRepository,
    pub(crate) leistungen: InstrumentedLeistungRepository,
    pub(crate) rechnungen: InstrumentedRechnungRepository,
    pub(crate) seminare: InstrumentedSeminarRepository,
    pub(crate) seminar_termine: InstrumentedSeminarTerminRepository,
    pub(crate) uow: Box<dyn UnitOfWorkImpl>,
}

impl RepoStorage {
    pub(crate) fn new(uow: Box<dyn UnitOfWorkImpl>) -> Pin<Box<Self>> {
        let mut pinned = Box::pin(RepoStorage {
            klienten: InstrumentedKlientRepository::disabled(),
            haustiere: InstrumentedHaustierRepository::disabled(),
            produkte: InstrumentedProduktRepository::disabled(),
            behandlungen: InstrumentedBehandlungRepository::disabled(),
            leistungen: InstrumentedLeistungRepository::disabled(),
            rechnungen: InstrumentedRechnungRepository::disabled(),
            seminare: InstrumentedSeminarRepository::disabled(),
            seminar_termine: InstrumentedSeminarTerminRepository::disabled(),
            uow,
        });

        unsafe {
            let this = pinned.as_mut().get_unchecked_mut();
            let uow_ptr = std::ptr::from_ref(this.uow.as_ref());
            this.klienten = InstrumentedKlientRepository::new(uow_ptr);
            this.haustiere = InstrumentedHaustierRepository::new(uow_ptr);
            this.produkte = InstrumentedProduktRepository::new(uow_ptr);
            this.behandlungen = InstrumentedBehandlungRepository::new(uow_ptr);
            this.leistungen = InstrumentedLeistungRepository::new(uow_ptr);
            this.rechnungen = InstrumentedRechnungRepository::new(uow_ptr);
            this.seminare = InstrumentedSeminarRepository::new(uow_ptr);
            this.seminar_termine = InstrumentedSeminarTerminRepository::new(uow_ptr);
        }

        pinned
    }
}
