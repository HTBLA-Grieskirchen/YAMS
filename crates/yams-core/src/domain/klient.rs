use chrono::NaiveDate;
use error_stack::{Report, ResultExt};
use uuid::Uuid;

use crate::{
    ResultReport,
    domain::{Adresse, EmailAdresse, Mobilnummer},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KlientId(pub Uuid);

#[derive(Debug, thiserror::Error)]
pub enum KlientFehler {
    #[error("name darf nicht leer sein")]
    NameLeer,
    #[error("klient konnte nicht erzeugt werden")]
    Konstruktion,
}

const CONSTRUCTING: &str = "while constructing klient";

/// Aggregate
#[derive(Debug, Clone)]
pub struct Klient {
    id: KlientId,
    vorname: String,
    nachname: String,
    geburtstag: NaiveDate,
    email: EmailAdresse,
    mobilnummer: Mobilnummer,
    kundennummer: u64,
    einwilligung: bool,
    adresse: Adresse,
}

impl Klient {
    pub fn neu(id: KlientId, neu: NeuerKlient) -> ResultReport<Self, KlientFehler> {
        if neu.vorname.trim().is_empty() || neu.nachname.trim().is_empty() {
            return Err(Report::new(KlientFehler::NameLeer).attach(CONSTRUCTING));
        }
        Ok(Self {
            id,
            vorname: neu.vorname,
            nachname: neu.nachname,
            geburtstag: neu.geburtstag,
            email: neu.email,
            mobilnummer: neu.mobilnummer,
            kundennummer: neu.kundennummer,
            einwilligung: neu.einwilligung,
            adresse: neu.adresse,
        })
    }

    pub fn from_parts(
        id: KlientId,
        vorname: String,
        nachname: String,
        geburtstag: NaiveDate,
        email: impl AsRef<str>,
        mobilnummer: impl AsRef<str>,
        kundennummer: u64,
        einwilligung: bool,
        adresse: Adresse,
    ) -> ResultReport<Self, KlientFehler> {
        let email = EmailAdresse::new(email)
            .change_context(KlientFehler::Konstruktion)
            .attach(CONSTRUCTING)?;
        let mobilnummer = Mobilnummer::new(mobilnummer)
            .change_context(KlientFehler::Konstruktion)
            .attach(CONSTRUCTING)?;
        Self::neu(
            id,
            NeuerKlient::neu(
                vorname,
                nachname,
                geburtstag,
                email,
                mobilnummer,
                kundennummer,
                einwilligung,
                adresse,
            ),
        )
    }

    pub fn id(&self) -> &KlientId {
        &self.id
    }

    pub fn vorname(&self) -> &str {
        &self.vorname
    }

    pub fn nachname(&self) -> &str {
        &self.nachname
    }

    pub fn geburtstag(&self) -> NaiveDate {
        self.geburtstag
    }

    pub fn email(&self) -> &EmailAdresse {
        &self.email
    }

    pub fn mobilnummer(&self) -> &Mobilnummer {
        &self.mobilnummer
    }

    pub fn kundennummer(&self) -> u64 {
        self.kundennummer
    }

    pub fn einwilligung(&self) -> bool {
        self.einwilligung
    }

    pub fn adresse(&self) -> &Adresse {
        &self.adresse
    }
}

#[derive(Debug)]
pub struct NeuerKlient {
    vorname: String,
    nachname: String,
    geburtstag: NaiveDate,
    email: EmailAdresse,
    mobilnummer: Mobilnummer,
    kundennummer: u64,
    einwilligung: bool,
    adresse: Adresse,
}

impl NeuerKlient {
    pub fn neu(
        vorname: impl Into<String>,
        nachname: impl Into<String>,
        geburtstag: NaiveDate,
        email: EmailAdresse,
        mobilnummer: Mobilnummer,
        kundennummer: u64,
        einwilligung: bool,
        adresse: Adresse,
    ) -> Self {
        Self {
            vorname: vorname.into(),
            nachname: nachname.into(),
            geburtstag,
            email,
            mobilnummer,
            kundennummer,
            einwilligung,
            adresse,
        }
    }

    pub fn vorname(&self) -> &str {
        &self.vorname
    }

    pub fn nachname(&self) -> &str {
        &self.nachname
    }

    pub fn geburtstag(&self) -> NaiveDate {
        self.geburtstag
    }

    pub fn email(&self) -> &EmailAdresse {
        &self.email
    }

    pub fn mobilnummer(&self) -> &Mobilnummer {
        &self.mobilnummer
    }

    pub fn kundennummer(&self) -> u64 {
        self.kundennummer
    }

    pub fn einwilligung(&self) -> bool {
        self.einwilligung
    }

    pub fn adresse(&self) -> &Adresse {
        &self.adresse
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Ländercode;

    fn valid_adresse() -> Adresse {
        Adresse {
            postleitzahl: "4711".into(),
            stadt: "Grieskirchen".into(),
            straße_und_hausnummer: "Hauptstraße 1".into(),
            ländercode: Ländercode::DE,
        }
    }

    fn neu(
        vorname: &str,
        nachname: &str,
        email: EmailAdresse,
        mobil: Mobilnummer,
    ) -> ResultReport<Klient, KlientFehler> {
        Klient::neu(
            KlientId(Uuid::new_v4()),
            NeuerKlient::neu(
                vorname,
                nachname,
                NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                email,
                mobil,
                1001,
                true,
                valid_adresse(),
            ),
        )
    }

    #[test]
    fn klient_rejects_empty_vorname() {
        let err = neu(
            "",
            "Muster",
            "anna@muster.de".try_into().unwrap(),
            "1234567890".try_into().unwrap(),
        )
        .unwrap_err();
        assert!(matches!(err.current_context(), KlientFehler::NameLeer));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }

    #[test]
    fn klient_rejects_invalid_email_with_attach() {
        let err = Klient::from_parts(
            KlientId(Uuid::new_v4()),
            "Anna".into(),
            "Muster".into(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            "not-an-email",
            "1234567890",
            1001,
            true,
            valid_adresse(),
        )
        .unwrap_err();
        assert!(matches!(err.current_context(), KlientFehler::Konstruktion));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }

    #[test]
    fn klient_rejects_invalid_mobilnummer_with_attach() {
        let err = Klient::from_parts(
            KlientId(Uuid::new_v4()),
            "Anna".into(),
            "Muster".into(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            "anna@muster.de",
            "123",
            1001,
            true,
            valid_adresse(),
        )
        .unwrap_err();
        assert!(matches!(err.current_context(), KlientFehler::Konstruktion));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }
}
