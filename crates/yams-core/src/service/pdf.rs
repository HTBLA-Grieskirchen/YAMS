use crate::ResultReport;
use crate::domain::{
    Klient, Preis, RechnungId, RechnungIn, Seminar, SeminarBuchung, SeminarBuchungId,
    SeminarTerminAbgehalten, SeminarTerminId,
};
use crate::ports::{
    Klientbericht, ObjectStore, ObjectStoreError, ObjectStream, PdfDokument, Rechnungsbericht,
    Rechnungspositionsbericht, Teilnahmebestätigung,
};
use crate::service::praxis::praxis;

pub fn rechnung_object_key(id: &RechnungId) -> String {
    format!("rechnungen/{}.pdf", id.0)
}

pub fn teilnahme_object_key(termin_id: &SeminarTerminId, buchung_id: &SeminarBuchungId) -> String {
    format!(
        "teilnahmebestaetigungen/{}/{}.pdf",
        termin_id.0, buchung_id.0
    )
}

pub async fn rechnung_pdf_laden(
    store: &dyn ObjectStore,
    id: &RechnungId,
) -> ResultReport<Option<ObjectStream>, ObjectStoreError> {
    store.get(&rechnung_object_key(id)).await
}

pub async fn teilnahme_pdf_laden(
    store: &dyn ObjectStore,
    termin_id: &SeminarTerminId,
    buchung_id: &SeminarBuchungId,
) -> ResultReport<Option<ObjectStream>, ObjectStoreError> {
    store
        .get(&teilnahme_object_key(termin_id, buchung_id))
        .await
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn rechnung_key_uses_uuid() {
        let id = RechnungId(Uuid::nil());
        assert_eq!(
            rechnung_object_key(&id),
            "rechnungen/00000000-0000-0000-0000-000000000000.pdf"
        );
    }

    #[test]
    fn teilnahme_key_nests_termin_and_buchung() {
        let termin = SeminarTerminId(Uuid::nil());
        let buchung = SeminarBuchungId(Uuid::from_u128(1));
        assert_eq!(
            teilnahme_object_key(&termin, &buchung),
            "teilnahmebestaetigungen/00000000-0000-0000-0000-000000000000/00000000-0000-0000-0000-000000000001.pdf"
        );
    }
}
