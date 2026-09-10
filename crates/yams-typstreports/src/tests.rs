use std::sync::OnceLock;

use chrono::{NaiveDate, TimeZone, Utc};
use rust_decimal::Decimal;
use typst::layout::{Frame, FrameItem};
use uuid::Uuid;
use yams_core::domain::{
    Adresse, EmailAdresse, KlientId, Ländercode, Menge, Preis, Ratio, RechnungId, SeminarBuchungId,
    SeminarTerminId,
};
use yams_core::ports::{
    Klientbericht, PdfDokument, PraxisAngaben, Rechnungsbericht, Rechnungspositionsbericht,
    Teilnahmebestätigung,
};
use yams_core::service::praxis;

use super::{PagedDocument, TypstPdfRenderer, compile_paged};

struct Rendered {
    pdf: Vec<u8>,
    text: String,
    pages: usize,
}

fn renderer() -> &'static TypstPdfRenderer {
    static RENDERER: OnceLock<TypstPdfRenderer> = OnceLock::new();
    RENDERER.get_or_init(TypstPdfRenderer::new)
}

fn generate(dokument: &PdfDokument) -> Rendered {
    let engine = renderer().engine.lock().expect("typst engine mutex");
    let doc = compile_paged(&engine, dokument).expect("typst compile");
    let text = document_text(&doc);
    let pages = doc.pages().len();
    let pdf = typst_pdf::pdf(&doc, &typst_pdf::PdfOptions::default()).expect("typst pdf export");
    Rendered { pdf, text, pages }
}

fn document_text(doc: &PagedDocument) -> String {
    let mut out = String::new();
    for page in doc.pages() {
        collect_frame_text(&page.frame, &mut out);
    }
    out
}

fn collect_frame_text(frame: &Frame, out: &mut String) {
    for (_, item) in frame.items() {
        match item {
            FrameItem::Group(group) => collect_frame_text(&group.frame, out),
            FrameItem::Text(text) => out.push_str(&text.text),
            _ => {}
        }
    }
}

fn assert_valid_pdf(rendered: &Rendered) {
    assert!(
        rendered.pdf.starts_with(b"%PDF"),
        "export is not a PDF ({} bytes)",
        rendered.pdf.len()
    );
    assert!(
        rendered.pdf.len() > 200,
        "PDF too small: {} bytes",
        rendered.pdf.len()
    );
    assert!(rendered.pages >= 1);
}

fn assert_contains(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            text.contains(needle),
            "missing {needle:?} in rendered text:\n{text}"
        );
    }
}

fn assert_absent(text: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            !text.contains(needle),
            "unexpected {needle:?} in rendered text:\n{text}"
        );
    }
}

fn praxis_angaben() -> PraxisAngaben {
    praxis()
}

fn adresse() -> Adresse {
    Adresse {
        postleitzahl: "4711".into(),
        stadt: "Grieskirchen".into(),
        straße_und_hausnummer: "Hauptstraße 1".into(),
        ländercode: Ländercode::AT,
    }
}

fn klient() -> Klientbericht {
    klient_named("Anna", "Muster", 1001)
}

fn klient_named(vorname: &str, nachname: &str, kundennummer: u64) -> Klientbericht {
    Klientbericht {
        id: KlientId(Uuid::nil()),
        vorname: vorname.into(),
        nachname: nachname.into(),
        kundennummer,
        adresse: adresse(),
        email: EmailAdresse::new("anna@muster.de").unwrap(),
    }
}

fn preis(amount: i64, scale: u32) -> Preis {
    Preis::new(Decimal::new(amount, scale)).unwrap()
}

fn menge(amount: i64, scale: u32) -> Menge {
    Menge::new(Decimal::new(amount, scale)).unwrap()
}

fn position(
    beschreibung: &str,
    einzelpreis: Preis,
    stückzahl: Menge,
    mwst: Ratio,
) -> Rechnungspositionsbericht {
    Rechnungspositionsbericht {
        beschreibung: beschreibung.into(),
        einzelpreis,
        stückzahl,
        mwst,
    }
}

fn rechnung(
    rechnungsnummer: u64,
    rechnungsdatum: NaiveDate,
    klient: Klientbericht,
    positionen: Vec<Rechnungspositionsbericht>,
    netto: Preis,
    mwst: Preis,
    brutto: Preis,
) -> PdfDokument {
    PdfDokument::Rechnung(Rechnungsbericht {
        rechnung_id: RechnungId(Uuid::nil()),
        rechnungsnummer,
        rechnungsdatum,
        praxis: praxis_angaben(),
        klient,
        positionen,
        gesamt_netto: netto,
        gesamt_mwst: mwst,
        gesamt_brutto: brutto,
    })
}

fn sample_rechnung() -> PdfDokument {
    rechnung(
        42,
        NaiveDate::from_ymd_opt(2026, 8, 23).unwrap(),
        klient(),
        vec![position(
            "Futter",
            preis(25, 0),
            menge(2, 0),
            Ratio::new(Decimal::new(19, 2)).unwrap(),
        )],
        preis(50, 0),
        preis(95, 1),
        preis(595, 1),
    )
}

fn teilnahme(
    titel: &str,
    klient: Klientbericht,
    beginn: chrono::DateTime<Utc>,
    ende: chrono::DateTime<Utc>,
    ort_name: Option<String>,
    ort_adresse: Option<Adresse>,
) -> PdfDokument {
    PdfDokument::Teilnahmebestätigung(Teilnahmebestätigung {
        termin_id: SeminarTerminId(Uuid::nil()),
        buchung_id: SeminarBuchungId(Uuid::from_u128(1)),
        praxis: praxis_angaben(),
        klient,
        seminar_titel: titel.into(),
        zeitraum_beginn: beginn,
        zeitraum_ende: ende,
        ort_name,
        ort_adresse,
    })
}

fn sample_teilnahme() -> PdfDokument {
    teilnahme(
        "Hufseminar",
        klient(),
        Utc.with_ymd_and_hms(2026, 8, 25, 10, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 8, 25, 16, 0, 0).unwrap(),
        Some("Hof".into()),
        None,
    )
}

fn hof_adresse() -> Adresse {
    Adresse {
        postleitzahl: "4720".into(),
        stadt: "Kelz".into(),
        straße_und_hausnummer: "Hofweg 9".into(),
        ländercode: Ländercode::AT,
    }
}

#[test_log::test]
fn renders_rechnung_pdf() {
    let rendered = generate(&sample_rechnung());
    assert_valid_pdf(&rendered);
    assert_contains(
        &rendered.text,
        &[
            "Rechnung Nr.",
            "42",
            "23.08.2026",
            "Anna",
            "Muster",
            "Futter",
            "Energetik Sabine Petschl",
            "19 %",
        ],
    );
}

#[test_log::test]
fn renders_teilnahme_pdf() {
    let rendered = generate(&sample_teilnahme());
    assert_valid_pdf(&rendered);
    assert_contains(
        &rendered.text,
        &[
            "Teilnahmebestätigung",
            "Anna",
            "Muster",
            "Hufseminar",
            "25.08.2026",
            "Ort:",
            "Hof",
        ],
    );
    assert_absent(&rendered.text, &["Adresse:"]);
}

#[test_log::test]
fn rechnung_empty_positionen_still_renders() {
    let rendered = generate(&rechnung(
        1,
        NaiveDate::from_ymd_opt(2026, 1, 5).unwrap(),
        klient(),
        vec![],
        Preis::zero(),
        Preis::zero(),
        Preis::zero(),
    ));
    assert_valid_pdf(&rendered);
    assert_contains(
        &rendered.text,
        &["Beschreibung", "05.01.2026", "Netto:", "MwSt:", "Brutto:"],
    );
}

#[test_log::test]
fn rechnung_zero_vat_and_zero_totals() {
    let rendered = generate(&rechnung(
        7,
        NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
        klient(),
        vec![position(
            "Geschenk",
            Preis::zero(),
            Menge::one(),
            Ratio::zero(),
        )],
        Preis::zero(),
        Preis::zero(),
        Preis::zero(),
    ));
    assert_valid_pdf(&rendered);
    assert_contains(&rendered.text, &["Geschenk", "0 %"]);
}

#[test_log::test]
fn rechnung_full_vat_and_fractional_quantity() {
    let rendered = generate(&rechnung(
        8,
        NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
        klient(),
        vec![position("Öl", preis(10, 0), menge(15, 1), Ratio::one())],
        preis(15, 0),
        preis(15, 0),
        preis(30, 0),
    ));
    assert_valid_pdf(&rendered);
    assert_contains(&rendered.text, &["Öl", "1.5", "100 %"]);
}

#[test_log::test]
fn rechnung_formats_cents_with_comma() {
    let rendered = generate(&rechnung(
        9,
        NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
        klient(),
        vec![position(
            "Beratung",
            preis(1250, 2),
            Menge::one(),
            Ratio::new(Decimal::new(20, 2)).unwrap(),
        )],
        preis(1250, 2),
        preis(250, 2),
        preis(15, 0),
    ));
    assert_valid_pdf(&rendered);
    assert_contains(&rendered.text, &["12,5 €", "20 %"]);
}

#[test_log::test]
fn rechnung_survives_umlauts_and_typst_markup_in_fields() {
    let mut klient = klient_named("Jörg", "Müller-Straße", 9);
    klient.adresse.straße_und_hausnummer = "Gasse #1 * 2 $x$ [y]".into();
    let rendered = generate(&rechnung(
        10,
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
        klient,
        vec![position(
            "Hörnchen #futter *not-bold*",
            preis(1, 0),
            Menge::one(),
            Ratio::zero(),
        )],
        preis(1, 0),
        Preis::zero(),
        preis(1, 0),
    ));
    assert_valid_pdf(&rendered);
    assert_contains(
        &rendered.text,
        &[
            "Jörg",
            "Müller-Straße",
            "Gasse #1 * 2 $x$ [y]",
            "Hörnchen #futter *not-bold*",
            "29.02.2024",
        ],
    );
}

#[test_log::test]
fn rechnung_many_positions_span_pages() {
    let positionen = (0..40)
        .map(|i| {
            position(
                &format!("Position {i:02} mit längerer Beschreibung"),
                preis(1, 0),
                Menge::one(),
                Ratio::zero(),
            )
        })
        .collect();
    let rendered = generate(&rechnung(
        11,
        NaiveDate::from_ymd_opt(2026, 6, 1).unwrap(),
        klient(),
        positionen,
        preis(40, 0),
        Preis::zero(),
        preis(40, 0),
    ));
    assert_valid_pdf(&rendered);
    assert!(
        rendered.pages >= 2,
        "expected pagination, got {} page(s)",
        rendered.pages
    );
    assert_contains(&rendered.text, &["Position 00", "Position 39"]);
}

#[test_log::test]
fn teilnahme_omits_location_when_unset() {
    let rendered = generate(&teilnahme(
        "Hufseminar",
        klient(),
        Utc.with_ymd_and_hms(2026, 8, 25, 10, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 8, 25, 16, 0, 0).unwrap(),
        None,
        None,
    ));
    assert_valid_pdf(&rendered);
    assert_contains(&rendered.text, &["Zeitraum:", "25.08.2026"]);
    assert_absent(&rendered.text, &["Ort:", "Adresse:"]);
}

#[test_log::test]
fn teilnahme_shows_address_without_name() {
    let rendered = generate(&teilnahme(
        "Hufseminar",
        klient(),
        Utc.with_ymd_and_hms(2026, 8, 25, 10, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 8, 25, 16, 0, 0).unwrap(),
        None,
        Some(hof_adresse()),
    ));
    assert_valid_pdf(&rendered);
    assert_contains(&rendered.text, &["Adresse:", "Hofweg 9", "4720", "Kelz"]);
    assert_absent(&rendered.text, &["Ort:"]);
}

#[test_log::test]
fn teilnahme_shows_name_and_address() {
    let rendered = generate(&teilnahme(
        "Hufseminar",
        klient(),
        Utc.with_ymd_and_hms(2026, 8, 25, 10, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 8, 26, 16, 0, 0).unwrap(),
        Some("Hof".into()),
        Some(hof_adresse()),
    ));
    assert_valid_pdf(&rendered);
    assert_contains(
        &rendered.text,
        &[
            "Ort:",
            "Hof",
            "Adresse:",
            "Hofweg 9",
            "25.08.2026",
            "26.08.2026",
        ],
    );
}

#[test_log::test]
fn teilnahme_keeps_markup_characters_in_title() {
    let rendered = generate(&teilnahme(
        "Kurs #1: *Hufe* $a$ [AT]",
        klient_named("D'Angelo", "Straße", 2),
        Utc.with_ymd_and_hms(2026, 8, 25, 10, 0, 0).unwrap(),
        Utc.with_ymd_and_hms(2026, 8, 25, 16, 0, 0).unwrap(),
        Some("Hof".into()),
        None,
    ));
    assert_valid_pdf(&rendered);
    assert_contains(
        &rendered.text,
        &["Kurs #1: *Hufe* $a$ [AT]", "D'Angelo", "Straße"],
    );
}
