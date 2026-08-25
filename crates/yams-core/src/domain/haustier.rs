use chrono::NaiveDate;
use error_stack::Report;
use uuid::Uuid;

use crate::{ResultReport, domain::KlientId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HaustierId(pub Uuid);

#[derive(Debug, thiserror::Error)]
pub enum HaustierFehler {
    #[error("name darf nicht leer sein")]
    NameLeer,
}

const CONSTRUCTING: &str = "while constructing haustier";

/// Aggregat
#[derive(Debug, Clone)]
pub struct Haustier {
    id: HaustierId,
    klient_id: KlientId,
    name: String,
    geburtstag: NaiveDate,
    tierart: String,
    beschreibung: String,
}

impl Haustier {
    pub fn neu(id: HaustierId, neu: NeuesHaustier) -> ResultReport<Self, HaustierFehler> {
        if neu.name.trim().is_empty() {
            return Err(Report::new(HaustierFehler::NameLeer).attach(CONSTRUCTING));
        }
        Ok(Self {
            id,
            klient_id: neu.klient_id,
            name: neu.name,
            geburtstag: neu.geburtstag,
            tierart: neu.tierart,
            beschreibung: neu.beschreibung,
        })
    }

    pub fn from_parts(
        id: HaustierId,
        klient_id: KlientId,
        name: String,
        geburtstag: NaiveDate,
        tierart: String,
        beschreibung: String,
    ) -> ResultReport<Self, HaustierFehler> {
        Self::neu(
            id,
            NeuesHaustier::neu(klient_id, name, geburtstag, tierart, beschreibung),
        )
    }

    pub fn id(&self) -> &HaustierId {
        &self.id
    }

    pub fn klient_id(&self) -> &KlientId {
        &self.klient_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn geburtstag(&self) -> NaiveDate {
        self.geburtstag
    }

    pub fn tierart(&self) -> &str {
        &self.tierart
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }
}

#[derive(Debug)]
pub struct NeuesHaustier {
    klient_id: KlientId,
    name: String,
    geburtstag: NaiveDate,
    tierart: String,
    beschreibung: String,
}

impl NeuesHaustier {
    pub fn neu(
        klient_id: KlientId,
        name: impl Into<String>,
        geburtstag: NaiveDate,
        tierart: impl Into<String>,
        beschreibung: impl Into<String>,
    ) -> Self {
        Self {
            klient_id,
            name: name.into(),
            geburtstag,
            tierart: tierart.into(),
            beschreibung: beschreibung.into(),
        }
    }

    pub fn klient_id(&self) -> &KlientId {
        &self.klient_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn geburtstag(&self) -> NaiveDate {
        self.geburtstag
    }

    pub fn tierart(&self) -> &str {
        &self.tierart
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn neues(name: &str, klient_id: KlientId) -> NeuesHaustier {
        NeuesHaustier::neu(
            klient_id,
            name,
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            "Hund",
            "Mischling",
        )
    }

    #[test]
    fn haustier_rejects_empty_name() {
        let err = Haustier::neu(
            HaustierId(Uuid::new_v4()),
            neues("", KlientId(Uuid::new_v4())),
        )
        .unwrap_err();
        assert!(matches!(err.current_context(), HaustierFehler::NameLeer));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }

    #[test]
    fn haustier_neu_keeps_klient_id() {
        let klient_id = KlientId(Uuid::new_v4());
        let haustier = Haustier::neu(
            HaustierId(Uuid::new_v4()),
            neues("Bello", klient_id.clone()),
        )
        .unwrap();
        assert_eq!(haustier.klient_id(), &klient_id);
        assert_eq!(haustier.name(), "Bello");
    }

    #[test]
    fn haustier_from_parts_rejects_empty_name() {
        let err = Haustier::from_parts(
            HaustierId(Uuid::new_v4()),
            KlientId(Uuid::new_v4()),
            "   ".into(),
            NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
            "Hund".into(),
            "Mischling".into(),
        )
        .unwrap_err();
        assert!(matches!(err.current_context(), HaustierFehler::NameLeer));
    }
}
