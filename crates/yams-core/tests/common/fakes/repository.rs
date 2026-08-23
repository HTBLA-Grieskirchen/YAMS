use std::{
    ops::DerefMut,
    sync::{Arc, Mutex, MutexGuard},
};

use async_trait::async_trait;
use chrono::NaiveDate;
use rustc_hash::{FxHashMap, FxHashSet};
use uuid::Uuid;
use yams_core::{
    domain::{
        Behandlung, BehandlungId, Leistung, Rechnung, Haustier, HaustierId, Klient,
        KlientId, LeistungId, LeistungOffen, Produkt, ProduktId, RechnungOffen,
        behandlung::NeueBehandlung,
        haustier::NeuesHaustier,
        klient::NeuerKlient,
        leistung::{LeistungOffen as LeistungOffenType, NeueLeistung},
        produkt::NeuesProdukt,
    },
    ports::{
        BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository,
        ProduktRepository, RechnungRepository, RepositoryError, RepositoryResult,
    },
    uow::Versioned,
};

pub struct FakeDatastore {
    pub klienten: Mutex<FxHashMap<Uuid, Versioned<Klient>>>,
    pub haustiere: Mutex<FxHashMap<Uuid, Versioned<Haustier>>>,
    pub produkte: Mutex<FxHashMap<Uuid, Versioned<Produkt>>>,
    pub behandlungen: Mutex<FxHashMap<Uuid, Versioned<Behandlung>>>,
    pub leistungen: Mutex<FxHashMap<Uuid, Versioned<Leistung>>>,
    pub rechnungen: Mutex<FxHashMap<Uuid, Versioned<Rechnung>>>,
}

impl Clone for FakeDatastore {
    fn clone(&self) -> Self {
        Self {
            klienten: Mutex::new(self.klienten.lock().unwrap().clone()),
            haustiere: Mutex::new(self.haustiere.lock().unwrap().clone()),
            produkte: Mutex::new(self.produkte.lock().unwrap().clone()),
            behandlungen: Mutex::new(self.behandlungen.lock().unwrap().clone()),
            leistungen: Mutex::new(self.leistungen.lock().unwrap().clone()),
            rechnungen: Mutex::new(self.rechnungen.lock().unwrap().clone()),
        }
    }
}

impl FakeDatastore {
    pub fn new() -> Self {
        Self {
            klienten: Mutex::new(FxHashMap::default()),
            haustiere: Mutex::new(FxHashMap::default()),
            produkte: Mutex::new(FxHashMap::default()),
            behandlungen: Mutex::new(FxHashMap::default()),
            leistungen: Mutex::new(FxHashMap::default()),
            rechnungen: Mutex::new(FxHashMap::default()),
        }
    }

    pub fn replace_with(&self, other: &FakeDatastore) {
        *self.klienten.lock().unwrap() = other.klienten.lock().unwrap().clone();
        *self.haustiere.lock().unwrap() = other.haustiere.lock().unwrap().clone();
        *self.produkte.lock().unwrap() = other.produkte.lock().unwrap().clone();
        *self.behandlungen.lock().unwrap() = other.behandlungen.lock().unwrap().clone();
        *self.leistungen.lock().unwrap() = other.leistungen.lock().unwrap().clone();
        *self.rechnungen.lock().unwrap() = other.rechnungen.lock().unwrap().clone();
    }

    pub fn merge(
        target: &FakeDatastore,
        reference: &FakeDatastore,
        tx: &FakeDatastore,
    ) -> Result<FakeDatastore, RepositoryError> {
        fn acquire_map_lock<T>(
            map: &'_ Mutex<FxHashMap<Uuid, Versioned<T>>>,
        ) -> Result<MutexGuard<'_, FxHashMap<Uuid, Versioned<T>>>, RepositoryError> {
            map.lock().map_err(|_| RepositoryError::OperationFailed)
        }

        let mut target_klienten = acquire_map_lock(&target.klienten)?;
        let mut target_haustiere = acquire_map_lock(&target.haustiere)?;
        let mut target_produkte = acquire_map_lock(&target.produkte)?;
        let mut target_behandlungen = acquire_map_lock(&target.behandlungen)?;
        let mut target_leistungen = acquire_map_lock(&target.leistungen)?;
        let mut target_rechnungen = acquire_map_lock(&target.rechnungen)?;

        FakeDatastore::merge_single_aggregate(
            target_klienten.deref_mut(),
            &*acquire_map_lock(&reference.klienten)?,
            &*acquire_map_lock(&tx.klienten)?,
        )?;
        FakeDatastore::merge_single_aggregate(
            target_haustiere.deref_mut(),
            &*acquire_map_lock(&reference.haustiere)?,
            &*acquire_map_lock(&tx.haustiere)?,
        )?;
        FakeDatastore::merge_single_aggregate(
            target_produkte.deref_mut(),
            &*acquire_map_lock(&reference.produkte)?,
            &*acquire_map_lock(&tx.produkte)?,
        )?;
        FakeDatastore::merge_single_aggregate(
            target_behandlungen.deref_mut(),
            &*acquire_map_lock(&reference.behandlungen)?,
            &*acquire_map_lock(&tx.behandlungen)?,
        )?;
        FakeDatastore::merge_single_aggregate(
            target_leistungen.deref_mut(),
            &*acquire_map_lock(&reference.leistungen)?,
            &*acquire_map_lock(&tx.leistungen)?,
        )?;
        FakeDatastore::merge_single_aggregate(
            target_rechnungen.deref_mut(),
            &*acquire_map_lock(&reference.rechnungen)?,
            &*acquire_map_lock(&tx.rechnungen)?,
        )?;

        Ok(FakeDatastore {
            klienten: Mutex::new(target_klienten.clone()),
            haustiere: Mutex::new(target_haustiere.clone()),
            produkte: Mutex::new(target_produkte.clone()),
            behandlungen: Mutex::new(target_behandlungen.clone()),
            leistungen: Mutex::new(target_leistungen.clone()),
            rechnungen: Mutex::new(target_rechnungen.clone()),
        })
    }

    fn merge_single_aggregate<T: Clone>(
        target: &mut FxHashMap<Uuid, Versioned<T>>,
        reference: &FxHashMap<Uuid, Versioned<T>>,
        tx: &FxHashMap<Uuid, Versioned<T>>,
    ) -> Result<(), RepositoryError> {
        let all_ids = reference
            .keys()
            .chain(tx.keys())
            .cloned()
            .collect::<FxHashSet<_>>();

        for id in all_ids {
            let target_versioned = target.get(&id).cloned();
            let reference_versioned = reference.get(&id).cloned();
            let tx_versioned = tx.get(&id).cloned();

            match (reference_versioned, tx_versioned) {
                (Some(reference_versioned), Some(tx_versioned)) => {
                    if reference_versioned.v() != tx_versioned.v() {
                        let Some(target_versioned) = target_versioned else {
                            return Err(RepositoryError::Conflict);
                        };
                        if target_versioned.v() > reference_versioned.v() {
                            return Err(RepositoryError::VersionMismatch {
                                expected: reference_versioned.v(),
                                actual: Some(target_versioned.v()),
                            });
                        }
                        target.insert(id, tx_versioned);
                    }
                }
                (Some(reference_versioned), None) => {
                    if let Some(target_v) = target_versioned
                        && target_v.v() > reference_versioned.v()
                    {
                        return Err(RepositoryError::VersionMismatch {
                            expected: reference_versioned.v(),
                            actual: Some(target_v.v()),
                        });
                    }
                    target.remove(&id);
                }
                (None, Some(tx_versioned)) => {
                    if target_versioned.is_some() {
                        return Err(RepositoryError::Conflict);
                    }
                    target.insert(id, tx_versioned);
                }
                _ => {}
            }
        }
        Ok(())
    }
}

pub struct FakeKlientenRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeKlientenRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl KlientRepository for FakeKlientenRepository {
    async fn find_by_id(&self, id: KlientId) -> RepositoryResult<Versioned<Klient>> {
        let data = self.datastore.klienten.lock().unwrap();
        Ok(data.get(&id.0).cloned().ok_or(RepositoryError::NotFound)?)
    }

    async fn create(&self, klient: NeuerKlient) -> RepositoryResult<Versioned<Klient>> {
        let id = KlientId(Uuid::new_v4());
        let mut data = self.datastore.klienten.lock().unwrap();
        let versioned = Versioned::init(Klient {
            id,
            vorname: klient.vorname,
            nachname: klient.nachname,
            geburtstag: klient.geburtstag,
            email: klient.email,
            mobilnummer: klient.mobilnummer,
            kundennummer: klient.kundennummer,
            einwilligung: klient.einwilligung,
            adresse: klient.adresse,
        });
        data.insert(versioned.id.0.clone(), versioned.clone());
        Ok(versioned)
    }

    async fn update(&self, klient: &mut Versioned<Klient>) -> RepositoryResult<()> {
        let mut data = self.datastore.klienten.lock().unwrap();
        if let Some(existing) = data.get(&klient.id.0) {
            if existing.v() != klient.v() {
                Err(RepositoryError::VersionMismatch {
                    expected: existing.v(),
                    actual: Some(klient.v()),
                })?;
            }
            *klient = klient.clone().incremented();
            data.insert(klient.id.0.clone(), klient.clone());
            return Ok(());
        }
        Err(RepositoryError::NotFound)?
    }

    async fn delete(&self, klient: Versioned<Klient>) -> RepositoryResult<()> {
        let mut data = self.datastore.klienten.lock().unwrap();
        if let Some(existing) = data.get(&klient.id.0) {
            if existing.v() != klient.v() {
                Err(RepositoryError::VersionMismatch {
                    expected: existing.v(),
                    actual: Some(klient.v()),
                })?;
            }
            data.remove(&klient.id.0);
            return Ok(());
        }
        Err(RepositoryError::NotFound)?
    }
}

pub struct FakeHaustiereRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeHaustiereRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl HaustierRepository for FakeHaustiereRepository {
    async fn find_by_id(&self, id: HaustierId) -> RepositoryResult<Versioned<Haustier>> {
        let data = self.datastore.haustiere.lock().unwrap();
        Ok(data.get(&id.0).cloned().ok_or(RepositoryError::NotFound)?)
    }

    async fn find_by_klient_id(&self, klient_id: KlientId) -> RepositoryResult<Vec<Versioned<Haustier>>> {
        let data = self.datastore.haustiere.lock().unwrap();
        Ok(data
            .values()
            .filter(|h| h.klient_id == klient_id)
            .cloned()
            .collect())
    }

    async fn find_all(&self) -> RepositoryResult<Vec<Versioned<Haustier>>> {
        let data = self.datastore.haustiere.lock().unwrap();
        Ok(data.values().cloned().collect())
    }

    async fn create(&self, haustier: NeuesHaustier) -> RepositoryResult<Versioned<Haustier>> {
        let id = HaustierId(Uuid::new_v4());
        let mut data = self.datastore.haustiere.lock().unwrap();
        let versioned = Versioned::init(Haustier {
            id,
            klient_id: haustier.klient_id,
            name: haustier.name,
            geburtstag: haustier.geburtstag,
            tierart: haustier.tierart,
            beschreibung: haustier.beschreibung,
        });
        data.insert(versioned.id.0.clone(), versioned.clone());
        Ok(versioned)
    }

    async fn update(&self, haustier: &mut Versioned<Haustier>) -> RepositoryResult<()> {
        let mut data = self.datastore.haustiere.lock().unwrap();
        if let Some(existing) = data.get(&haustier.id.0) {
            if existing.v() != haustier.v() {
                Err(RepositoryError::VersionMismatch {
                    expected: existing.v(),
                    actual: Some(haustier.v()),
                })?;
            }
            *haustier = haustier.clone().incremented();
            data.insert(haustier.id.0.clone(), haustier.clone());
            return Ok(());
        }
        Err(RepositoryError::NotFound)?
    }

    async fn delete(&self, haustier: Versioned<Haustier>) -> RepositoryResult<()> {
        let mut data = self.datastore.haustiere.lock().unwrap();
        if let Some(existing) = data.get(&haustier.id.0) {
            if existing.v() != haustier.v() {
                Err(RepositoryError::VersionMismatch {
                    expected: existing.v(),
                    actual: Some(haustier.v()),
                })?;
            }
            data.remove(&haustier.id.0);
            return Ok(());
        }
        Err(RepositoryError::NotFound)?
    }
}

pub struct FakeProdukteRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeProdukteRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl ProduktRepository for FakeProdukteRepository {
    async fn find_by_id(&self, id: ProduktId) -> RepositoryResult<Versioned<Produkt>> {
        let data = self.datastore.produkte.lock().unwrap();
        Ok(data.get(&id.0).cloned().ok_or(RepositoryError::NotFound)?)
    }

    async fn create(&self, produkt: NeuesProdukt) -> RepositoryResult<Versioned<Produkt>> {
        let id = ProduktId(Uuid::new_v4());
        let mut data = self.datastore.produkte.lock().unwrap();
        let versioned = Versioned::init(Produkt {
            id,
            name: produkt.name,
            beschreibung: produkt.beschreibung,
            einzelpreis: produkt.einzelpreis,
            mwst_prozentsatz: produkt.mwst_prozentsatz,
        });
        data.insert(versioned.id.0.clone(), versioned.clone());
        Ok(versioned)
    }
}

pub struct FakeBehandlungenRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeBehandlungenRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl BehandlungRepository for FakeBehandlungenRepository {
    async fn find_by_id(&self, id: BehandlungId) -> RepositoryResult<Versioned<Behandlung>> {
        let data = self.datastore.behandlungen.lock().unwrap();
        Ok(data.get(&id.0).cloned().ok_or(RepositoryError::NotFound)?)
    }

    async fn create(&self, behandlung: NeueBehandlung) -> RepositoryResult<Versioned<Behandlung>> {
        let id = BehandlungId(Uuid::new_v4());
        let mut data = self.datastore.behandlungen.lock().unwrap();
        let versioned = Versioned::init(Behandlung {
            id,
            name: behandlung.name,
            beschreibung: behandlung.beschreibung,
            standardpreis: behandlung.standardpreis,
            mwst_prozentsatz: behandlung.mwst_prozentsatz,
        });
        data.insert(versioned.id.0.clone(), versioned.clone());
        Ok(versioned)
    }
}

pub struct FakeLeistungenRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeLeistungenRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl LeistungRepository for FakeLeistungenRepository {
    async fn create(&self, leistung: NeueLeistung) -> RepositoryResult<Versioned<LeistungOffen>> {
        let id = LeistungId(Uuid::new_v4());
        let mut data = self.datastore.leistungen.lock().unwrap();
        let offen = LeistungOffenType::neu(id, leistung);
        let versioned = Versioned::init(Leistung::Offen(offen.clone()));
        data.insert(offen.id().0, versioned.clone());
        Ok(Versioned::new(versioned.v(), offen))
    }

    async fn find_offene_by_datum(
        &self,
        datum: NaiveDate,
    ) -> RepositoryResult<Vec<Versioned<LeistungOffen>>> {
        let data = self.datastore.leistungen.lock().unwrap();
        Ok(data
            .values()
            .filter_map(|versioned| match &**versioned {
                Leistung::Offen(leistung) if leistung.leistungsdatum() == datum => {
                    Some(Versioned::new(versioned.v(), leistung.clone()))
                }
                _ => None,
            })
            .collect())
    }

    async fn update(&self, leistung: &mut Versioned<Leistung>) -> RepositoryResult<()> {
        let mut data = self.datastore.leistungen.lock().unwrap();
        let id = leistung.id().0;
        if let Some(existing) = data.get(&id) {
            if existing.v() != leistung.v() {
                Err(RepositoryError::VersionMismatch {
                    expected: existing.v(),
                    actual: Some(leistung.v()),
                })?;
            }
            *leistung = leistung.clone().incremented();
            data.insert(id, leistung.clone());
            return Ok(());
        }
        Err(RepositoryError::NotFound)?
    }
}

pub struct FakeRechnungenRepository {
    datastore: Arc<FakeDatastore>,
}

impl FakeRechnungenRepository {
    pub fn new(datastore: Arc<FakeDatastore>) -> Self {
        Self { datastore }
    }
}

#[async_trait]
impl RechnungRepository for FakeRechnungenRepository {
    async fn create(&self, rechnung: RechnungOffen) -> RepositoryResult<Versioned<RechnungOffen>> {
        let mut data = self.datastore.rechnungen.lock().unwrap();
        let versioned = Versioned::init(Rechnung::Offen(rechnung.clone()));
        data.insert(rechnung.id().0, versioned);
        Ok(Versioned::init(rechnung))
    }

    async fn naechste_rechnungsnummer(&self) -> RepositoryResult<u64> {
        let data = self.datastore.rechnungen.lock().unwrap();
        let max = data
            .values()
            .map(|rechnung| rechnung.rechnungsnummer())
            .max()
            .unwrap_or(0);
        Ok(max + 1)
    }

    async fn find_by_klient_id(
        &self,
        klient_id: KlientId,
    ) -> RepositoryResult<Vec<Versioned<Rechnung>>> {
        let data = self.datastore.rechnungen.lock().unwrap();
        Ok(data
            .values()
            .filter(|r| r.klient_id() == &klient_id)
            .cloned()
            .collect())
    }
}
