use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Ratio muss zwischen 0 und 1 liegen: {0}")]
pub struct RatioFehler(pub Decimal);

/// Dimensionless ratio in `0..=1` (100% = 1). MwSt-Anteil and Rabatt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ratio(Decimal);

impl Ratio {
    pub fn new(value: Decimal) -> Result<Self, RatioFehler> {
        if value < Decimal::ZERO || value > Decimal::ONE {
            Err(RatioFehler(value))
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
    fn ratio_accepts_zero_and_one() {
        assert_eq!(Ratio::new(Decimal::ZERO).unwrap().value(), Decimal::ZERO);
        assert_eq!(Ratio::new(Decimal::ONE).unwrap().value(), Decimal::ONE);
    }

    #[test_log::test]
    fn ratio_rejects_greater_than_one() {
        assert!(Ratio::new(Decimal::new(101, 2)).is_err());
        assert!(Ratio::new(Decimal::new(2, 0)).is_err());
    }

    #[test_log::test]
    fn ratio_rejects_negative() {
        assert!(Ratio::new(Decimal::new(-1, 2)).is_err());
    }
}
