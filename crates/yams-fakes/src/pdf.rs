use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use yams_core::{
    ResultReport,
    ports::{PdfDokument, PdfRenderError, PdfRenderer},
};

pub const FAKE_PDF: &[u8] = b"%PDF-1.4\n%yams-fake\n%%EOF\n";

#[derive(Clone, Default)]
pub struct FakePdfRenderer {
    calls: Arc<Mutex<Vec<PdfDokument>>>,
}

impl FakePdfRenderer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn calls(&self) -> Vec<PdfDokument> {
        self.calls.lock().expect("fake pdf renderer mutex").clone()
    }

    pub fn canned_bytes(&self) -> &'static [u8] {
        FAKE_PDF
    }
}

#[async_trait]
impl PdfRenderer for FakePdfRenderer {
    async fn rendern(&self, dokument: &PdfDokument) -> ResultReport<Vec<u8>, PdfRenderError> {
        self.calls
            .lock()
            .expect("fake pdf renderer mutex")
            .push(dokument.clone());
        Ok(FAKE_PDF.to_vec())
    }
}
