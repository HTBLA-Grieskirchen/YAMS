use crate::domain::{Klient, Preis, RechnungIn, Seminar, SeminarBuchung, SeminarTerminAbgehalten};
use crate::ports::{
    Klientbericht, PdfDokument, Rechnungsbericht, Rechnungspositionsbericht, Teilnahmebestätigung,
};
use crate::service::praxis::praxis;

pub fn klient_bericht(klient: &Klient) -> Klientbericht {
    Klientbericht {
        id: klient.id().clone(),
        vorname: klient.vorname().to_string(),
        nachname: klient.nachname().to_string(),
        kundennummer: klient.kundennummer(),
        adresse: klient.adresse().clone(),
        email: klient.email().clone(),
    }
}

pub fn rechnungsdokument<S>(rechnung: &RechnungIn<S>, klient: &Klient) -> PdfDokument {
    let gesamt_mwst = rechnung
        .positionen()
        .iter()
        .fold(Preis::zero(), |acc, position| acc + position.mwst_betrag());
    PdfDokument::Rechnung(Rechnungsbericht {
        rechnung_id: rechnung.id().clone(),
        rechnungsnummer: rechnung.rechnungsnummer(),
        rechnungsdatum: rechnung.rechnungsdatum(),
        praxis: praxis(),
        klient: klient_bericht(klient),
        positionen: rechnung
            .positionen()
            .iter()
            .map(|position| Rechnungspositionsbericht {
                beschreibung: position.beschreibung().to_string(),
                einzelpreis: position.einzelpreis().clone(),
                stückzahl: position.stückzahl().clone(),
                mwst: position.mwst().clone(),
            })
            .collect(),
        gesamt_netto: rechnung.gesamtbetrag_netto(),
        gesamt_mwst,
        gesamt_brutto: rechnung.gesamtbetrag_brutto(),
    })
}

pub fn teilnahme_dokument(
    termin: &SeminarTerminAbgehalten,
    seminar: &Seminar,
    buchung: &SeminarBuchung,
    klient: &Klient,
) -> PdfDokument {
    PdfDokument::Teilnahmebestätigung(Teilnahmebestätigung {
        termin_id: termin.id().clone(),
        buchung_id: buchung.id().clone(),
        praxis: praxis(),
        klient: klient_bericht(klient),
        seminar_titel: seminar.titel().to_string(),
        zeitraum_beginn: termin.zeitraum().beginn(),
        zeitraum_ende: termin.zeitraum().ende(),
        ort_name: termin.ort().ort_name().map(str::to_string),
        ort_adresse: termin.ort().adresse().cloned(),
    })
}
