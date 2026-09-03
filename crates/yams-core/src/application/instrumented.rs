use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;

use crate::ResultReport;
use crate::application::uow::UnitOfWorkImpl;
use crate::ports::{
    BehandlungRepository, HaustierRepository, KlientRepository, LeistungRepository, ObjectStore,
    ObjectStoreError, ObjectStream, PdfDokument, PdfRenderError, PdfRenderer, ProduktRepository,
    RechnungRepository, RepositoryResult, SeminarRepository, SeminarTerminRepository,
};

#[derive(Clone)]
pub struct InstrumentedObjectStore {
    inner: Arc<dyn ObjectStore>,
}

impl InstrumentedObjectStore {
    pub fn new(inner: Arc<dyn ObjectStore>) -> Arc<dyn ObjectStore> {
        Arc::new(Self { inner })
    }
}

#[async_trait]
impl ObjectStore for InstrumentedObjectStore {
    #[instrument(skip(self, bytes), fields(key, bytes_len = bytes.len()), level = "debug", err(Debug))]
    async fn put(&self, key: &str, bytes: &[u8]) -> ResultReport<(), ObjectStoreError> {
        self.inner.put(key, bytes).await
    }

    #[instrument(skip(self), fields(key), level = "debug", err(Debug))]
    async fn get(&self, key: &str) -> ResultReport<Option<ObjectStream>, ObjectStoreError> {
        self.inner.get(key).await
    }

    #[instrument(skip(self), fields(key), level = "debug", err(Debug))]
    async fn delete(&self, key: &str) -> ResultReport<(), ObjectStoreError> {
        self.inner.delete(key).await
    }
}

#[derive(Clone)]
pub struct InstrumentedPdfRenderer {
    inner: Arc<dyn PdfRenderer>,
}

impl InstrumentedPdfRenderer {
    pub fn new(inner: Arc<dyn PdfRenderer>) -> Arc<dyn PdfRenderer> {
        Arc::new(Self { inner })
    }
}

fn pdf_dokument_kind(dokument: &PdfDokument) -> &'static str {
    match dokument {
        PdfDokument::Rechnung(_) => "rechnung",
        PdfDokument::Teilnahmebestätigung(_) => "teilnahme",
    }
}

#[async_trait]
impl PdfRenderer for InstrumentedPdfRenderer {
    #[instrument(
        skip(self, dokument),
        fields(kind = pdf_dokument_kind(dokument)),
        level = "debug",
        err(Debug)
    )]
    async fn rendern(&self, dokument: &PdfDokument) -> ResultReport<Vec<u8>, PdfRenderError> {
        self.inner.rendern(dokument).await
    }
}

pub struct InstrumentedUnitOfWork {
    inner: Option<Box<dyn UnitOfWorkImpl>>,
}

impl InstrumentedUnitOfWork {
    pub fn wrap(inner: Box<dyn UnitOfWorkImpl>) -> Box<dyn UnitOfWorkImpl> {
        Box::new(Self {
            inner: Some(inner),
        })
    }

    fn inner(&self) -> &dyn UnitOfWorkImpl {
        self.inner
            .as_deref()
            .expect("unit of work already concluded")
    }
}

#[async_trait]
impl UnitOfWorkImpl for InstrumentedUnitOfWork {
    #[instrument(skip(self), level = "debug", err(Debug))]
    async fn commit(mut self: Box<Self>) -> RepositoryResult<()> {
        let inner = self.inner.take().expect("unit of work already concluded");
        inner.commit().await
    }

    #[instrument(skip(self), level = "debug", err(Debug))]
    async fn rollback(mut self: Box<Self>) -> RepositoryResult<()> {
        let inner = self.inner.take().expect("unit of work already concluded");
        inner.rollback().await
    }

    fn klienten(&self) -> &dyn KlientRepository {
        self.inner().klienten()
    }

    fn haustiere(&self) -> &dyn HaustierRepository {
        self.inner().haustiere()
    }

    fn produkte(&self) -> &dyn ProduktRepository {
        self.inner().produkte()
    }

    fn behandlungen(&self) -> &dyn BehandlungRepository {
        self.inner().behandlungen()
    }

    fn leistungen(&self) -> &dyn LeistungRepository {
        self.inner().leistungen()
    }

    fn rechnungen(&self) -> &dyn RechnungRepository {
        self.inner().rechnungen()
    }

    fn seminare(&self) -> &dyn SeminarRepository {
        self.inner().seminare()
    }

    fn seminar_termine(&self) -> &dyn SeminarTerminRepository {
        self.inner().seminar_termine()
    }
}
