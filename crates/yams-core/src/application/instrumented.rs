use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;

use crate::ResultReport;
use crate::ports::{
    ObjectStore, ObjectStoreError, ObjectStream, PdfDokument, PdfRenderError, PdfRenderer,
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
