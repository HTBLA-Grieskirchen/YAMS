use error_stack::Report;
use uuid::Uuid;

use crate::{
    ResultReport,
    domain::{Preis, Ratio},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProduktId(pub Uuid);

#[derive(Debug, thiserror::Error)]
pub enum ProduktFehler {
    #[error("name darf nicht leer sein")]
    NameLeer,
    #[error("produkt konnte nicht erzeugt werden")]
    Konstruktion,
}

const CONSTRUCTING: &str = "while constructing produkt";

/// Aggregate
#[derive(Debug, Clone)]
pub struct Produkt {
    id: ProduktId,
    name: String,
    beschreibung: String,
    einzelpreis: Preis,
    mwst: Ratio,
}

impl Produkt {
    pub fn neu(id: ProduktId, neu: NeuesProdukt) -> Self {
        Self {
            id,
            name: neu.name,
            beschreibung: neu.beschreibung,
            einzelpreis: neu.einzelpreis,
            mwst: neu.mwst,
        }
    }

    pub fn from_parts(
        id: ProduktId,
        name: String,
        beschreibung: String,
        einzelpreis: Preis,
        mwst: Ratio,
    ) -> ResultReport<Self, ProduktFehler> {
        Ok(Self::neu(
            id,
            NeuesProdukt::neu(name, beschreibung, einzelpreis, mwst)?,
        ))
    }

    pub fn id(&self) -> &ProduktId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn einzelpreis(&self) -> &Preis {
        &self.einzelpreis
    }

    pub fn mwst(&self) -> &Ratio {
        &self.mwst
    }
}

#[derive(Debug)]
pub struct NeuesProdukt {
    name: String,
    beschreibung: String,
    einzelpreis: Preis,
    mwst: Ratio,
}

impl NeuesProdukt {
    pub fn neu(
        name: impl Into<String>,
        beschreibung: impl Into<String>,
        einzelpreis: Preis,
        mwst: Ratio,
    ) -> ResultReport<Self, ProduktFehler> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(Report::new(ProduktFehler::NameLeer).attach(CONSTRUCTING));
        }
        Ok(Self {
            name,
            beschreibung: beschreibung.into(),
            einzelpreis,
            mwst,
        })
    }
}

impl NeuesProdukt {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn einzelpreis(&self) -> &Preis {
        &self.einzelpreis
    }

    pub fn mwst(&self) -> &Ratio {
        &self.mwst
    }
}

#[cfg(test)]
mod tests {
    use error_stack::ResultExt;
    use rust_decimal::Decimal;

    use super::*;

    fn preis() -> Preis {
        Preis::new(Decimal::new(25, 0)).unwrap()
    }

    #[test]
    fn produkt_rejects_empty_name() {
        let err = NeuesProdukt::neu("", "Futter", preis(), Ratio::zero()).unwrap_err();
        assert!(matches!(err.current_context(), ProduktFehler::NameLeer));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }

    #[test]
    fn produkt_accepts_zero_mwst() {
        let produkt = NeuesProdukt::neu("Futter", "Premium", preis(), Ratio::zero()).unwrap();
        assert_eq!(produkt.mwst().value(), Decimal::ZERO);
    }

    #[test]
    fn produkt_ratio_greater_than_one_cannot_exist() {
        assert!(Ratio::new(Decimal::new(101, 2)).is_err());
        let mwst = Ratio::new(Decimal::new(101, 2))
            .change_context(ProduktFehler::Konstruktion)
            .attach(CONSTRUCTING);
        assert!(mwst.is_err());
    }
}
