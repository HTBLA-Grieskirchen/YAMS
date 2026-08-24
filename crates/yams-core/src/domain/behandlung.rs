use error_stack::Report;
use uuid::Uuid;

use crate::{
    ResultReport,
    domain::{Preis, Ratio},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BehandlungId(pub Uuid);

#[derive(Debug, thiserror::Error)]
pub enum BehandlungFehler {
    #[error("name darf nicht leer sein")]
    NameLeer,
}

const CONSTRUCTING: &str = "while constructing behandlung";

/// Aggregate
#[derive(Debug, Clone)]
pub struct Behandlung {
    id: BehandlungId,
    name: String,
    beschreibung: String,
    standardpreis: Preis,
    mwst: Ratio,
}

impl Behandlung {
    pub fn neu(id: BehandlungId, neu: NeueBehandlung) -> Self {
        Self {
            id,
            name: neu.name,
            beschreibung: neu.beschreibung,
            standardpreis: neu.standardpreis,
            mwst: neu.mwst,
        }
    }

    pub fn from_parts(
        id: BehandlungId,
        name: String,
        beschreibung: String,
        standardpreis: Preis,
        mwst: Ratio,
    ) -> ResultReport<Self, BehandlungFehler> {
        Ok(Self::neu(
            id,
            NeueBehandlung::neu(name, beschreibung, standardpreis, mwst)?,
        ))
    }

    pub fn id(&self) -> &BehandlungId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn standardpreis(&self) -> &Preis {
        &self.standardpreis
    }

    pub fn mwst(&self) -> &Ratio {
        &self.mwst
    }
}

#[derive(Debug)]
pub struct NeueBehandlung {
    name: String,
    beschreibung: String,
    standardpreis: Preis,
    mwst: Ratio,
}

impl NeueBehandlung {
    pub fn neu(
        name: impl Into<String>,
        beschreibung: impl Into<String>,
        standardpreis: Preis,
        mwst: Ratio,
    ) -> ResultReport<Self, BehandlungFehler> {
        let name = name.into();
        if name.trim().is_empty() {
            return Err(Report::new(BehandlungFehler::NameLeer).attach(CONSTRUCTING));
        }
        Ok(Self {
            name,
            beschreibung: beschreibung.into(),
            standardpreis,
            mwst,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn beschreibung(&self) -> &str {
        &self.beschreibung
    }

    pub fn standardpreis(&self) -> &Preis {
        &self.standardpreis
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
        Preis::new(Decimal::new(50, 0)).unwrap()
    }

    #[test]
    fn behandlung_rejects_empty_name() {
        let err = NeueBehandlung::neu("", "Untersuchung", preis(), Ratio::one()).unwrap_err();
        assert!(matches!(err.current_context(), BehandlungFehler::NameLeer));
        assert!(format!("{err:?}").contains(CONSTRUCTING));
    }

    #[test]
    fn behandlung_accepts_zero_mwst() {
        let behandlung = NeueBehandlung::neu("Untersuchung", "Allgemein", preis(), Ratio::zero())
            .unwrap();
        assert_eq!(behandlung.mwst().value(), Decimal::ZERO);
    }

    #[test]
    fn behandlung_accepts_full_mwst() {
        let behandlung =
            NeueBehandlung::neu("Untersuchung", "Allgemein", preis(), Ratio::one()).unwrap();
        assert_eq!(behandlung.mwst().value(), Decimal::ONE);
    }
}
