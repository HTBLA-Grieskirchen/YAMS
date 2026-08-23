use std::str::FromStr;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::{
    domain::{
        BehandlungId, HaustierId, KlientId, LeistungQuelle, LeistungStatus, Preis, ProduktId,
        RechnungId, RechnungStatus,
    },
    ports::RepositoryError,
};

pub fn parse_naive_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}

pub fn parse_uuid(s: &str) -> Result<Uuid, RepositoryError> {
    Uuid::from_str(s).map_err(|_| RepositoryError::Data)
}

pub fn parse_preis(s: &str) -> Result<Preis, RepositoryError> {
    let decimal = Decimal::from_str(s).map_err(|_| RepositoryError::Data)?;
    Preis::new(decimal).map_err(|_| RepositoryError::Data)
}

pub fn leistung_status_from_str(s: &str) -> Result<LeistungStatus, RepositoryError> {
    match s {
        "offen" => Ok(LeistungStatus::Offen),
        "abgerechnet" => Ok(LeistungStatus::Abgerechnet),
        _ => Err(RepositoryError::Data),
    }
}

pub fn leistung_status_to_str(status: &LeistungStatus) -> &'static str {
    match status {
        LeistungStatus::Offen => "offen",
        LeistungStatus::Abgerechnet => "abgerechnet",
    }
}

pub fn rechnung_status_from_str(s: &str) -> Result<RechnungStatus, RepositoryError> {
    match s {
        "offen" => Ok(RechnungStatus::Offen),
        "bezahlt" => Ok(RechnungStatus::Bezahlt),
        _ => Err(RepositoryError::Data),
    }
}

pub fn rechnung_status_to_str(status: &RechnungStatus) -> &'static str {
    match status {
        RechnungStatus::Offen => "offen",
        RechnungStatus::Bezahlt => "bezahlt",
    }
}

pub fn quelle_from_row(
    quelle_typ: &str,
    quelle_id: Option<String>,
) -> Result<LeistungQuelle, RepositoryError> {
    match quelle_typ {
        "produkt" => {
            let id = quelle_id.ok_or(RepositoryError::Data)?;
            let uuid = parse_uuid(&id)?;
            Ok(LeistungQuelle::Produkt(ProduktId(uuid)))
        }
        "behandlung" => {
            let id = quelle_id.ok_or(RepositoryError::Data)?;
            let uuid = parse_uuid(&id)?;
            Ok(LeistungQuelle::Behandlung(BehandlungId(uuid)))
        }
        "manuell" => Ok(LeistungQuelle::Manuell),
        _ => Err(RepositoryError::Data),
    }
}

pub fn quelle_to_db(quelle: &LeistungQuelle) -> (&'static str, Option<String>) {
    match quelle {
        LeistungQuelle::Produkt(id) => ("produkt", Some(id.0.to_string())),
        LeistungQuelle::Behandlung(id) => ("behandlung", Some(id.0.to_string())),
        LeistungQuelle::Manuell => ("manuell", None),
    }
}

pub fn preis_to_str(preis: &Preis) -> String {
    preis.value().to_string()
}

pub fn format_naive_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn parse_klient_id(s: &str) -> Result<KlientId, RepositoryError> {
    Ok(KlientId(parse_uuid(s)?))
}

pub fn parse_haustier_id(s: &str) -> Result<HaustierId, RepositoryError> {
    Ok(HaustierId(parse_uuid(s)?))
}

pub fn parse_rechnung_id(s: &str) -> Result<RechnungId, RepositoryError> {
    Ok(RechnungId(parse_uuid(s)?))
}
