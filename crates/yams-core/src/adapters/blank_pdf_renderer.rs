use async_trait::async_trait;
use tracing::debug;

use crate::ResultReport;
use crate::ports::{PdfDokument, PdfRenderError, PdfRenderer};

/// Minimal valid one-page PDF with empty content. Used as the App default.
pub const BLANK_PDF: &[u8] = b"%PDF-1.1\n\
1 0 obj<< /Type /Catalog /Pages 2 0 R >>endobj\n\
2 0 obj<< /Type /Pages /Kids [3 0 R] /Count 1 >>endobj\n\
3 0 obj<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] >>endobj\n\
trailer<< /Root 1 0 R >>\n\
%%EOF\n";

pub struct BlankPdfRenderer;

#[async_trait]
impl PdfRenderer for BlankPdfRenderer {
    async fn rendern(&self, _dokument: &PdfDokument) -> ResultReport<Vec<u8>, PdfRenderError> {
        debug!(bytes_len = BLANK_PDF.len(), "blank pdf renderer");
        Ok(BLANK_PDF.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Adresse, EmailAdresse, KlientId, Ländercode, Menge, Preis, RechnungId};
    use crate::ports::{Klientbericht, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht};
    use chrono::NaiveDate;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    fn dummy_rechnung() -> PdfDokument {
        PdfDokument::Rechnung(Rechnungsbericht {
            rechnung_id: RechnungId(Uuid::nil()),
            rechnungsnummer: 1,
            rechnungsdatum: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            praxis: PraxisAngaben {
                name: "Praxis".into(),
                straße_und_hausnummer: "A 1".into(),
                postleitzahl: "4710".into(),
                stadt: "Grieskirchen".into(),
                ländercode: Ländercode::AT,
                email: "a@b.at".into(),
                telefon: "+431234567".into(),
                uid: "ATU00000000".into(),
            },
            klient: Klientbericht {
                id: KlientId(Uuid::nil()),
                vorname: "A".into(),
                nachname: "B".into(),
                kundennummer: 1,
                adresse: Adresse {
                    postleitzahl: "4710".into(),
                    stadt: "Grieskirchen".into(),
                    straße_und_hausnummer: "B 2".into(),
                    ländercode: Ländercode::AT,
                },
                email: EmailAdresse::new("a@b.at").unwrap(),
            },
            positionen: vec![Rechnungspositionsbericht {
                beschreibung: "x".into(),
                einzelpreis: Preis::new(Decimal::ONE).unwrap(),
                stückzahl: Menge::one(),
                mwst: crate::domain::Ratio::zero(),
            }],
            gesamt_netto: Preis::new(Decimal::ONE).unwrap(),
            gesamt_mwst: Preis::zero(),
            gesamt_brutto: Preis::new(Decimal::ONE).unwrap(),
        })
    }

    #[test_log::test(pollster::test)]
    async fn blank_renderer_returns_pdf_prefix() {
        let bytes = BlankPdfRenderer.rendern(&dummy_rechnung()).await.unwrap();
        assert!(bytes.starts_with(b"%PDF"));
    }
}
