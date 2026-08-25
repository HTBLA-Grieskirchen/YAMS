use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};

use crate::ResultReport;
use crate::domain::{
    Adresse, EmailAdresse, KlientId, Menge, Preis, Ratio, RechnungId, SeminarBuchungId,
    SeminarTerminId,
};

#[derive(thiserror::Error, Debug)]
pub enum PdfRenderError {
    #[error("pdf rendering failed")]
    Rendering,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PraxisAngaben {
    pub name: String,
    pub straße_und_hausnummer: String,
    pub postleitzahl: String,
    pub stadt: String,
    pub ländercode: crate::domain::Ländercode,
    pub email: String,
    pub telefon: String,
    pub uid: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Klientbericht {
    pub id: KlientId,
    pub vorname: String,
    pub nachname: String,
    pub kundennummer: u64,
    pub adresse: Adresse,
    pub email: EmailAdresse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rechnungspositionsbericht {
    pub beschreibung: String,
    pub einzelpreis: Preis,
    pub stückzahl: Menge,
    pub mwst: Ratio,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rechnungsbericht {
    pub rechnung_id: RechnungId,
    pub rechnungsnummer: u64,
    pub rechnungsdatum: NaiveDate,
    pub praxis: PraxisAngaben,
    pub klient: Klientbericht,
    pub positionen: Vec<Rechnungspositionsbericht>,
    pub gesamt_netto: Preis,
    pub gesamt_mwst: Preis,
    pub gesamt_brutto: Preis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Teilnahmebestätigung {
    pub termin_id: SeminarTerminId,
    pub buchung_id: SeminarBuchungId,
    pub praxis: PraxisAngaben,
    pub klient: Klientbericht,
    pub seminar_titel: String,
    pub zeitraum_beginn: DateTime<Utc>,
    pub zeitraum_ende: DateTime<Utc>,
    pub ort_name: Option<String>,
    pub ort_adresse: Option<Adresse>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PdfDokument {
    Rechnung(Rechnungsbericht),
    Teilnahmebestätigung(Teilnahmebestätigung),
}

#[async_trait]
pub trait PdfRenderer: Send + Sync {
    async fn rendern(&self, dokument: &PdfDokument) -> ResultReport<Vec<u8>, PdfRenderError>;
}
