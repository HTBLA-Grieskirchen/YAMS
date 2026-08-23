use std::str::FromStr;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::{
    domain::{
        BehandlungId, HaustierId, KlientId, LeistungQuelle, Preis, ProduktId, RechnungId,
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
    let decimal = parse_decimal(s)?;
    Preis::new(decimal).map_err(|_| RepositoryError::Data)
}

pub fn parse_decimal(s: &str) -> Result<Decimal, RepositoryError> {
    Decimal::from_str(s).map_err(|_| RepositoryError::Data)
}

pub fn quelle_from_row(
    quelle_typ: &str,
    quelle_id: Option<String>,
    quelle_menge: Option<String>,
    quelle_einzelpreis: Option<String>,
    quelle_preis: Option<String>,
) -> Result<LeistungQuelle, RepositoryError> {
    match quelle_typ {
        "produkt" => {
            let id = quelle_id.ok_or(RepositoryError::Data)?;
            let uuid = parse_uuid(&id)?;
            let menge_str = quelle_menge.ok_or(RepositoryError::Data)?;
            let menge = parse_decimal(&menge_str)?;
            let einzelpreis_str = quelle_einzelpreis.ok_or(RepositoryError::Data)?;
            let einzelpreis = parse_preis(&einzelpreis_str)?;
            Ok(LeistungQuelle::Produkt {
                produkt_id: ProduktId(uuid),
                menge,
                einzelpreis,
            })
        }
        "behandlung" => {
            let id = quelle_id.ok_or(RepositoryError::Data)?;
            let uuid = parse_uuid(&id)?;
            let preis_str = quelle_preis.ok_or(RepositoryError::Data)?;
            let preis = parse_preis(&preis_str)?;
            Ok(LeistungQuelle::Behandlung {
                behandlung_id: BehandlungId(uuid),
                preis,
            })
        }
        "manuell" => {
            let preis_str = quelle_preis.ok_or(RepositoryError::Data)?;
            let preis = parse_preis(&preis_str)?;
            Ok(LeistungQuelle::Manuell { preis })
        }
        _ => Err(RepositoryError::Data),
    }
}

pub struct QuelleDbColumns {
    pub typ: &'static str,
    pub id: Option<String>,
    pub menge: Option<String>,
    pub einzelpreis: Option<String>,
    pub preis: Option<String>,
}

pub fn quelle_to_db(quelle: &LeistungQuelle) -> QuelleDbColumns {
    match quelle {
        LeistungQuelle::Produkt {
            produkt_id,
            menge,
            einzelpreis,
        } => QuelleDbColumns {
            typ: "produkt",
            id: Some(produkt_id.0.to_string()),
            menge: Some(decimal_to_str(menge)),
            einzelpreis: Some(preis_to_str(einzelpreis)),
            preis: None,
        },
        LeistungQuelle::Behandlung {
            behandlung_id,
            preis,
        } => QuelleDbColumns {
            typ: "behandlung",
            id: Some(behandlung_id.0.to_string()),
            menge: None,
            einzelpreis: None,
            preis: Some(preis_to_str(preis)),
        },
        LeistungQuelle::Manuell { preis } => QuelleDbColumns {
            typ: "manuell",
            id: None,
            menge: None,
            einzelpreis: None,
            preis: Some(preis_to_str(preis)),
        },
    }
}

pub fn preis_to_str(preis: &Preis) -> String {
    preis.value().to_string()
}

pub fn decimal_to_str(decimal: &Decimal) -> String {
    decimal.to_string()
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
