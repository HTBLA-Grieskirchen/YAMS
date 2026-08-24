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
    pub fn neu(id: ProduktId, neu: NeuesProdukt) -> ResultReport<Self, ProduktFehler> {
        if neu.name.trim().is_empty() {
            return Err(Report::new(ProduktFehler::NameLeer).attach(CONSTRUCTING));
        }
        Ok(Self {
            id,
            name: neu.name,
            beschreibung: neu.beschreibung,
            einzelpreis: neu.einzelpreis,
            mwst: neu.mwst,
        })
    }

    pub fn from_parts(
        id: ProduktId,
        name: String,
        beschreibung: String,
        einzelpreis: Preis,
        mwst: Ratio,
    ) -> ResultReport<Self, ProduktFehler> {
        Self::neu(id, NeuesProdukt::neu(name, beschreibung, einzelpreis, mwst))
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
    ) -> Self {
        Self {
            name: name.into(),
            beschreibung: beschreibung.into(),
            einzelpreis,
            mwst,
        }
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

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    use super::*;

    fn preis() -> Preis {
        Preis::new(Decimal::new(25, 0)).unwrap()
    }

    fn produkt(name: &str, mwst: Ratio) -> ResultReport<Produkt, ProduktFehler> {
        Produkt::neu(
            ProduktId(Uuid::new_v4()),
            NeuesProdukt::neu(name, "Premium", preis(), mwst),
        )
    }

    #[test]
    fn produkt_rejects_empty_name() {
        let err = produkt("", Ratio::zero()).unwrap_err();
        assert!(matches!(err.current_context(), ProduktFehler::NameLeer));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }

    #[test]
    fn produkt_accepts_zero_mwst() {
        let produkt = produkt("Futter", Ratio::zero()).unwrap();
        assert_eq!(produkt.mwst().value(), Decimal::ZERO);
    }

    #[test]
    fn produkt_ratio_greater_than_one_cannot_exist() {
        assert!(Ratio::new(Decimal::new(101, 2)).is_err());
    }
}
