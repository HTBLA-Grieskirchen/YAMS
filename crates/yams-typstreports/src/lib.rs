//! Typst-based `PdfRenderer` for Rechnung and Teilnahmebestätigung documents.

use std::sync::Mutex;

use async_trait::async_trait;
use chrono::{DateTime, Datelike, NaiveDate, Timelike, Utc};
use error_stack::{Report, ResultExt};
use rust_decimal::prelude::ToPrimitive;
use typst::foundations::{Array, Datetime, Dict, IntoValue, Value};
use typst_as_lib::TypstEngine;
use typst_as_lib::typst_kit_options::TypstKitFontOptions;
use typst_layout::PagedDocument;
use yams_core::{
    ResultReport,
    domain::{Adresse, Menge, Preis, Ratio},
    ports::{
        Klientbericht, PdfDokument, PdfRenderError, PdfRenderer, PraxisAngaben, Rechnungsbericht,
        Teilnahmebestätigung,
    },
};

static FORMAT_TYP: &str = include_str!("../templates/format.typ");
static RECHNUNG_TYP: &str = include_str!("../templates/rechnung.typ");
static TEILNAHME_TYP: &str = include_str!("../templates/teilnahme.typ");

pub struct TypstPdfRenderer {
    pub(crate) engine: Mutex<typst_as_lib::TypstEngine>,
}

impl TypstPdfRenderer {
    pub fn new() -> Self {
        let engine = TypstEngine::builder()
            .search_fonts_with(
                TypstKitFontOptions::default()
                    .include_system_fonts(false)
                    .include_embedded_fonts(true),
            )
            .with_static_source_file_resolver([
                ("format.typ", FORMAT_TYP),
                ("rechnung.typ", RECHNUNG_TYP),
                ("teilnahme.typ", TEILNAHME_TYP),
            ])
            .build();
        Self {
            engine: Mutex::new(engine),
        }
    }
}

impl Default for TypstPdfRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PdfRenderer for TypstPdfRenderer {
    async fn rendern(&self, dokument: &PdfDokument) -> ResultReport<Vec<u8>, PdfRenderError> {
        let engine = self.engine.lock().expect("typst engine mutex");
        let doc = compile_paged(&engine, dokument)?;
        typst_pdf::pdf(&doc, &typst_pdf::PdfOptions::default()).map_err(|err| {
            Report::new(PdfRenderError::Rendering)
                .attach(format!("{err:?}"))
                .attach("typst pdf export failed")
        })
    }
}

fn compile_paged(
    engine: &TypstEngine,
    dokument: &PdfDokument,
) -> ResultReport<PagedDocument, PdfRenderError> {
    let (main, inputs) = match dokument {
        PdfDokument::Rechnung(bericht) => ("rechnung.typ", rechnung_dict(bericht)),
        PdfDokument::Teilnahmebestätigung(bericht) => ("teilnahme.typ", teilnahme_dict(bericht)),
    };
    engine
        .compile_with_input(main, inputs)
        .output
        .change_context(PdfRenderError::Rendering)
        .attach("typst compile failed")
}

fn decimal(value: rust_decimal::Decimal) -> Value {
    value.to_f64().unwrap_or(0.0).into_value()
}

fn preis(value: &Preis) -> Value {
    decimal(value.value())
}

fn menge(value: &Menge) -> Value {
    decimal(value.value())
}

fn ratio(value: &Ratio) -> Value {
    decimal(value.value())
}

fn naive_date(value: NaiveDate) -> Value {
    Datetime::from_ymd(value.year(), value.month() as u8, value.day() as u8)
        .expect("valid invoice date")
        .into_value()
}

fn utc_datetime(value: DateTime<Utc>) -> Value {
    Datetime::from_ymd_hms(
        value.year(),
        value.month() as u8,
        value.day() as u8,
        value.hour() as u8,
        value.minute() as u8,
        value.second() as u8,
    )
    .expect("valid seminar datetime")
    .into_value()
}

fn adresse_dict(adresse: &Adresse) -> Value {
    let mut dict = Dict::new();
    dict.insert(
        "postleitzahl".into(),
        adresse.postleitzahl.clone().into_value(),
    );
    dict.insert("stadt".into(), adresse.stadt.clone().into_value());
    dict.insert(
        "straße_und_hausnummer".into(),
        adresse.straße_und_hausnummer.clone().into_value(),
    );
    dict.insert(
        "ländercode".into(),
        adresse.ländercode.as_str().into_value(),
    );
    dict.into_value()
}

fn praxis_dict(praxis: &PraxisAngaben) -> Value {
    let mut dict = Dict::new();
    dict.insert("name".into(), praxis.name.clone().into_value());
    dict.insert(
        "straße_und_hausnummer".into(),
        praxis.straße_und_hausnummer.clone().into_value(),
    );
    dict.insert(
        "postleitzahl".into(),
        praxis.postleitzahl.clone().into_value(),
    );
    dict.insert("stadt".into(), praxis.stadt.clone().into_value());
    dict.insert("ländercode".into(), praxis.ländercode.as_str().into_value());
    dict.insert("email".into(), praxis.email.clone().into_value());
    dict.insert("telefon".into(), praxis.telefon.clone().into_value());
    dict.insert("uid".into(), praxis.uid.clone().into_value());
    dict.into_value()
}

fn klient_dict(klient: &Klientbericht) -> Value {
    let mut dict = Dict::new();
    dict.insert("vorname".into(), klient.vorname.clone().into_value());
    dict.insert("nachname".into(), klient.nachname.clone().into_value());
    dict.insert(
        "kundennummer".into(),
        (klient.kundennummer as i64).into_value(),
    );
    dict.insert("adresse".into(), adresse_dict(&klient.adresse));
    dict.insert("email".into(), klient.email.as_ref().into_value());
    dict.into_value()
}

fn rechnung_dict(bericht: &Rechnungsbericht) -> Dict {
    let mut dict = Dict::new();
    dict.insert(
        "rechnungsnummer".into(),
        (bericht.rechnungsnummer as i64).into_value(),
    );
    dict.insert("rechnungsdatum".into(), naive_date(bericht.rechnungsdatum));
    dict.insert("praxis".into(), praxis_dict(&bericht.praxis));
    dict.insert("klient".into(), klient_dict(&bericht.klient));
    let positionen: Array = bericht
        .positionen
        .iter()
        .map(|position| {
            let mut row = Dict::new();
            row.insert(
                "beschreibung".into(),
                position.beschreibung.clone().into_value(),
            );
            row.insert("einzelpreis".into(), preis(&position.einzelpreis));
            row.insert("stückzahl".into(), menge(&position.stückzahl));
            row.insert("mwst".into(), ratio(&position.mwst));
            row.into_value()
        })
        .collect();
    dict.insert("positionen".into(), positionen.into_value());
    dict.insert("gesamt_netto".into(), preis(&bericht.gesamt_netto));
    dict.insert("gesamt_mwst".into(), preis(&bericht.gesamt_mwst));
    dict.insert("gesamt_brutto".into(), preis(&bericht.gesamt_brutto));
    dict
}

fn teilnahme_dict(bericht: &Teilnahmebestätigung) -> Dict {
    let mut dict = Dict::new();
    dict.insert("praxis".into(), praxis_dict(&bericht.praxis));
    dict.insert("klient".into(), klient_dict(&bericht.klient));
    dict.insert(
        "seminar_titel".into(),
        bericht.seminar_titel.clone().into_value(),
    );
    dict.insert(
        "zeitraum_beginn".into(),
        utc_datetime(bericht.zeitraum_beginn),
    );
    dict.insert("zeitraum_ende".into(), utc_datetime(bericht.zeitraum_ende));
    dict.insert(
        "ort_name".into(),
        match &bericht.ort_name {
            Some(name) => name.clone().into_value(),
            None => Value::None,
        },
    );
    dict.insert(
        "ort_adresse".into(),
        match &bericht.ort_adresse {
            Some(adresse) => adresse_dict(adresse),
            None => Value::None,
        },
    );
    dict
}

#[cfg(test)]
mod tests;
