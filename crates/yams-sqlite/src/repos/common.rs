use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::{
    domain::{
        BehandlungId, HaustierId, KlientId, LeistungQuelle, Menge, Preis, ProduktId, Ratio,
        RechnungId, SeminarBuchungId, SeminarTerminId,
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

pub fn parse_ratio(s: &str) -> Result<Ratio, RepositoryError> {
    let decimal = parse_decimal(s)?;
    Ratio::new(decimal).map_err(|_| RepositoryError::Data)
}

pub fn parse_menge(s: &str) -> Result<Menge, RepositoryError> {
    let decimal = parse_decimal(s)?;
    Menge::new(decimal).map_err(|_| RepositoryError::Data)
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
    quelle_mwst: Option<String>,
) -> Result<LeistungQuelle, RepositoryError> {
    let mwst_str = quelle_mwst.ok_or(RepositoryError::Data)?;
    let mwst = parse_ratio(&mwst_str)?;

    match quelle_typ {
        "produkt" => {
            let id = quelle_id.ok_or(RepositoryError::Data)?;
            let uuid = parse_uuid(&id)?;
            let menge_str = quelle_menge.ok_or(RepositoryError::Data)?;
            let menge = parse_menge(&menge_str)?;
            let einzelpreis_str = quelle_einzelpreis.ok_or(RepositoryError::Data)?;
            let einzelpreis = parse_preis(&einzelpreis_str)?;
            Ok(LeistungQuelle::Produkt {
                produkt_id: ProduktId(uuid),
                menge,
                einzelpreis,
                mwst,
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
                mwst,
            })
        }
        "manuell" => {
            let preis_str = quelle_preis.ok_or(RepositoryError::Data)?;
            let preis = parse_preis(&preis_str)?;
            Ok(LeistungQuelle::Manuell { preis, mwst })
        }
        "seminar" => {
            let termin_id = parse_uuid(&quelle_id.ok_or(RepositoryError::Data)?)?;
            let packed = quelle_menge.ok_or(RepositoryError::Data)?;
            let (buchung_str, rabatt_str) = packed.split_once('|').ok_or(RepositoryError::Data)?;
            let buchung_id = parse_uuid(buchung_str)?;
            let rabatt = parse_ratio(rabatt_str)?;
            let basis = parse_preis(&quelle_einzelpreis.ok_or(RepositoryError::Data)?)?;
            Ok(LeistungQuelle::Seminar {
                termin_id: SeminarTerminId(termin_id),
                buchung_id: SeminarBuchungId(buchung_id),
                teilnahmegebühr_basis: basis,
                rabatt,
                mwst,
            })
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
    pub mwst: String,
}

pub fn quelle_to_db(quelle: &LeistungQuelle) -> QuelleDbColumns {
    match quelle {
        LeistungQuelle::Produkt {
            produkt_id,
            menge,
            einzelpreis,
            mwst,
        } => QuelleDbColumns {
            typ: "produkt",
            id: Some(produkt_id.0.to_string()),
            menge: Some(menge_to_str(menge)),
            einzelpreis: Some(preis_to_str(einzelpreis)),
            preis: None,
            mwst: ratio_to_str(mwst),
        },
        LeistungQuelle::Behandlung {
            behandlung_id,
            preis,
            mwst,
        } => QuelleDbColumns {
            typ: "behandlung",
            id: Some(behandlung_id.0.to_string()),
            menge: None,
            einzelpreis: None,
            preis: Some(preis_to_str(preis)),
            mwst: ratio_to_str(mwst),
        },
        LeistungQuelle::Manuell { preis, mwst } => QuelleDbColumns {
            typ: "manuell",
            id: None,
            menge: None,
            einzelpreis: None,
            preis: Some(preis_to_str(preis)),
            mwst: ratio_to_str(mwst),
        },
        LeistungQuelle::Seminar {
            termin_id,
            buchung_id,
            teilnahmegebühr_basis,
            rabatt,
            mwst,
        } => QuelleDbColumns {
            typ: "seminar",
            id: Some(termin_id.0.to_string()),
            menge: Some(format!("{}|{}", buchung_id.0, ratio_to_str(rabatt))),
            einzelpreis: Some(preis_to_str(teilnahmegebühr_basis)),
            preis: None,
            mwst: ratio_to_str(mwst),
        },
    }
}

pub fn preis_to_str(preis: &Preis) -> String {
    preis.value().to_string()
}

pub fn ratio_to_str(ratio: &Ratio) -> String {
    ratio.value().to_string()
}

pub fn menge_to_str(menge: &Menge) -> String {
    menge.value().to_string()
}

pub fn format_naive_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

pub fn parse_datetime(s: &str) -> Result<DateTime<Utc>, RepositoryError> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| RepositoryError::Data)
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
