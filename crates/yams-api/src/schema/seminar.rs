use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::domain::{self, SeminarTermin};

use crate::schema::Adresse;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Seminar {
    pub id: Uuid,
    pub titel: String,
    pub beschreibung: String,
    pub teilnahmegebühr_basis: Decimal,
    pub mwst: Decimal,
    pub standarddauer_ms: Option<i64>,
}

pub fn schema_seminar_from_domain(seminar: domain::Seminar) -> Seminar {
    Seminar {
        id: seminar.id().0,
        titel: seminar.titel().to_string(),
        beschreibung: seminar.beschreibung().to_string(),
        teilnahmegebühr_basis: seminar.teilnahmegebühr_basis().value(),
        mwst: seminar.mwst().value(),
        standarddauer_ms: seminar.standarddauer().map(|d| d.num_milliseconds()),
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarOrt {
    pub ort_name: Option<String>,
    pub adresse: Option<Adresse>,
}

impl From<&domain::SeminarOrt> for SeminarOrt {
    fn from(value: &domain::SeminarOrt) -> Self {
        Self {
            ort_name: value.ort_name().map(str::to_string),
            adresse: value.adresse().cloned().map(Into::into),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SeminarTerminStatus {
    Geplant,
    Abgehalten,
    Abgesagt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Enum))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SeminarBuchungStatus {
    Bestätigt,
    Storniert,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarBuchung {
    pub id: Uuid,
    pub klient_id: Uuid,
    pub rabatt: Decimal,
    pub status: SeminarBuchungStatus,
    pub storniert_am: Option<DateTime<Utc>>,
    pub leistung_id: Option<Uuid>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarTerminDto {
    pub id: Uuid,
    pub seminar_id: Uuid,
    pub beginn: DateTime<Utc>,
    pub ende: DateTime<Utc>,
    pub ort: SeminarOrt,
    pub max_teilnehmer: Option<u32>,
    pub status: SeminarTerminStatus,
    pub abgehalten_am: Option<DateTime<Utc>>,
    pub abgesagt_am: Option<DateTime<Utc>>,
    pub absagegrund: Option<String>,
    pub buchungen: Vec<SeminarBuchung>,
}

pub fn schema_seminar_termin_from_domain(termin: SeminarTermin) -> SeminarTerminDto {
    let leistung_lookup = match &termin {
        SeminarTermin::Abgehalten(t) => Some(t.leistungen().clone()),
        _ => None,
    };

    let buchungen = termin
        .buchungen()
        .iter()
        .map(|buchung| {
            let leistung_id = leistung_lookup
                .as_ref()
                .and_then(|map| map.get(buchung.id()).map(|id| id.0));
            let (status, storniert_am) = match buchung.status() {
                domain::seminar_termin::SeminarBuchungStatus::Bestätigt => {
                    (SeminarBuchungStatus::Bestätigt, None)
                }
                domain::seminar_termin::SeminarBuchungStatus::Storniert { storniert_am } => {
                    (SeminarBuchungStatus::Storniert, Some(*storniert_am))
                }
            };
            SeminarBuchung {
                id: buchung.id().0,
                klient_id: buchung.klient_id().0,
                rabatt: buchung.rabatt().value(),
                status,
                storniert_am,
                leistung_id,
            }
        })
        .collect();

    let (status, abgehalten_am, abgesagt_am, absagegrund) = match &termin {
        SeminarTermin::Geplant(_) => (SeminarTerminStatus::Geplant, None, None, None),
        SeminarTermin::Abgehalten(t) => (
            SeminarTerminStatus::Abgehalten,
            Some(t.abgehalten_am()),
            None,
            None,
        ),
        SeminarTermin::Abgesagt(t) => (
            SeminarTerminStatus::Abgesagt,
            None,
            Some(t.abgesagt_am()),
            Some(t.grund().to_string()),
        ),
    };

    SeminarTerminDto {
        id: termin.id().0,
        seminar_id: termin.seminar_id().0,
        beginn: termin.zeitraum().beginn(),
        ende: termin.zeitraum().ende(),
        ort: SeminarOrt::from(termin.ort()),
        max_teilnehmer: termin.max_teilnehmer(),
        status,
        abgehalten_am,
        abgesagt_am,
        absagegrund,
        buchungen,
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct BuchungUmsatz {
    pub buchung_id: Uuid,
    pub klient_id: Uuid,
    pub netto: Decimal,
    pub mwst: Decimal,
    pub brutto: Decimal,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarUmsatzVorschau {
    pub termin_id: Uuid,
    pub seminar_id: Uuid,
    pub teilnehmer_anzahl: u32,
    pub positionen: Vec<BuchungUmsatz>,
    pub gesamt_netto: Decimal,
    pub gesamt_mwst: Decimal,
    pub gesamt_brutto: Decimal,
}

pub fn schema_umsatz_from_domain(
    umsatz: yams_core::service::SeminarUmsatzVorschauErgebnis,
) -> SeminarUmsatzVorschau {
    SeminarUmsatzVorschau {
        termin_id: umsatz.termin_id.0,
        seminar_id: umsatz.seminar_id.0,
        teilnehmer_anzahl: umsatz.teilnehmer_anzahl,
        positionen: umsatz
            .positionen
            .into_iter()
            .map(|p| BuchungUmsatz {
                buchung_id: p.buchung_id.0,
                klient_id: p.klient_id.0,
                netto: p.netto.value(),
                mwst: p.mwst.value(),
                brutto: p.brutto.value(),
            })
            .collect(),
        gesamt_netto: umsatz.gesamt_netto.value(),
        gesamt_mwst: umsatz.gesamt_mwst.value(),
        gesamt_brutto: umsatz.gesamt_brutto.value(),
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "openapi", derive(poem_openapi::Object))]
#[cfg_attr(feature = "openapi", oai(rename_all = "camelCase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SeminarUmsatzPrognose {
    pub stichtag: chrono::NaiveDate,
    pub termine: Vec<SeminarUmsatzVorschau>,
    pub gesamt_netto: Decimal,
    pub gesamt_brutto: Decimal,
}

pub fn schema_prognose_from_domain(
    prognose: yams_core::service::SeminarUmsatzPrognose,
) -> SeminarUmsatzPrognose {
    SeminarUmsatzPrognose {
        stichtag: prognose.stichtag,
        termine: prognose
            .termine
            .into_iter()
            .map(schema_umsatz_from_domain)
            .collect(),
        gesamt_netto: prognose.gesamt_netto.value(),
        gesamt_brutto: prognose.gesamt_brutto.value(),
    }
}
