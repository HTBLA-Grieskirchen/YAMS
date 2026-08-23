use chrono::NaiveDate;
use uuid::Uuid;

use crate::domain::{Adresse, EmailAdresse, Mobilnummer};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KlientId(pub Uuid);

/// Aggregat
#[derive(Debug, Clone)]
pub struct Klient {
    pub id: KlientId,
    pub vorname: String,
    pub nachname: String,
    pub geburtstag: NaiveDate,
    pub email: EmailAdresse,
    pub mobilnummer: Mobilnummer,
    pub kundennummer: i64,
    pub einwilligung: bool,
    pub adresse: Adresse,
}

pub struct NeuerKlient {
    pub vorname: String,
    pub nachname: String,
    pub geburtstag: NaiveDate,
    pub email: EmailAdresse,
    pub mobilnummer: Mobilnummer,
    pub kundennummer: i64,
    pub einwilligung: bool,
    pub adresse: Adresse,
}
