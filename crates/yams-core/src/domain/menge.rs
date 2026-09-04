use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Menge darf nicht negativ sein: {0}")]
pub struct MengeFehler(pub Decimal);

/// Unitless non-negative quantity (Stückzahl, gebuchte Menge).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Menge(Decimal);

impl Menge {
    pub fn new(value: Decimal) -> Result<Self, MengeFehler> {
        if value < Decimal::ZERO {
            Err(MengeFehler(value))
        } else {
            Ok(Self(value))
        }
    }

    pub fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    pub fn one() -> Self {
        Self(Decimal::ONE)
    }

    pub fn value(&self) -> Decimal {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn menge_accepts_zero() {
        assert_eq!(Menge::new(Decimal::ZERO).unwrap().value(), Decimal::ZERO);
    }

    #[test_log::test]
    fn menge_accepts_positive() {
        assert_eq!(
            Menge::new(Decimal::new(2, 0)).unwrap().value(),
            Decimal::new(2, 0)
        );
    }

    #[test_log::test]
    fn menge_rejects_negative() {
        assert!(Menge::new(Decimal::new(-1, 0)).is_err());
    }
}
