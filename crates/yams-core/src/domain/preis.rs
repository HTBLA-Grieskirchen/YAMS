use std::ops::{Add, Mul};

use rust_decimal::Decimal;
use thiserror::Error;

use super::{Menge, Ratio};

#[derive(Debug, Error)]
#[error("Preis darf nicht negativ sein: {0}")]
pub struct PreisFehler(pub Decimal);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Preis(Decimal);

impl Preis {
    pub fn new(value: Decimal) -> Result<Self, PreisFehler> {
        if value < Decimal::ZERO {
            Err(PreisFehler(value))
        } else {
            Ok(Self(value))
        }
    }

    pub fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    pub fn value(&self) -> Decimal {
        self.0
    }

    /// `basis * (1 - rabatt)`. Always non-negative because `Ratio` is `0..=1`.
    pub fn nach_rabatt(&self, rabatt: &Ratio) -> Preis {
        Preis(self.0 * (Decimal::ONE - rabatt.value()))
    }
}

impl Add for Preis {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<&Preis> for Preis {
    type Output = Self;

    fn add(self, rhs: &Preis) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Add<Preis> for &Preis {
    type Output = Preis;

    fn add(self, rhs: Preis) -> Self::Output {
        Preis(self.0 + rhs.0)
    }
}

impl Mul<&Menge> for &Preis {
    type Output = Preis;

    fn mul(self, rhs: &Menge) -> Self::Output {
        Preis(self.0 * rhs.value())
    }
}

impl Mul<&Ratio> for &Preis {
    type Output = Preis;

    fn mul(self, rhs: &Ratio) -> Self::Output {
        Preis(self.0 * rhs.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn preis_zero_is_zero() {
        assert_eq!(Preis::zero().value(), Decimal::ZERO);
    }

    #[test_log::test]
    fn preis_add_sums_values() {
        let a = Preis::new(Decimal::new(1250, 2)).unwrap();
        let b = Preis::new(Decimal::new(750, 2)).unwrap();

        assert_eq!((a + b).value(), Decimal::new(20, 0));
    }

    #[test_log::test]
    fn preis_times_menge_scales_value() {
        let preis = Preis::new(Decimal::new(25, 0)).unwrap();
        let menge = Menge::new(Decimal::new(2, 0)).unwrap();

        assert_eq!((&preis * &menge).value(), Decimal::new(50, 0));
    }

    #[test_log::test]
    fn preis_times_ratio_scales_value() {
        let preis = Preis::new(Decimal::new(100, 0)).unwrap();
        let mwst = Ratio::new(Decimal::new(20, 2)).unwrap();

        assert_eq!((&preis * &mwst).value(), Decimal::new(20, 0));
    }

    #[test_log::test]
    fn preis_new_rejects_negative() {
        assert!(Preis::new(Decimal::new(-1, 0)).is_err());
    }

    #[test_log::test]
    fn nach_rabatt_zero_keeps_basis() {
        let preis = Preis::new(Decimal::new(100, 0)).unwrap();
        assert_eq!(
            preis.nach_rabatt(&Ratio::zero()).value(),
            Decimal::new(100, 0)
        );
    }

    #[test_log::test]
    fn nach_rabatt_twenty_percent() {
        let preis = Preis::new(Decimal::new(100, 0)).unwrap();
        let rabatt = Ratio::new(Decimal::new(20, 2)).unwrap();
        assert_eq!(preis.nach_rabatt(&rabatt).value(), Decimal::new(80, 0));
    }

    #[test_log::test]
    fn nach_rabatt_full_is_zero() {
        let preis = Preis::new(Decimal::new(100, 0)).unwrap();
        assert_eq!(preis.nach_rabatt(&Ratio::one()).value(), Decimal::ZERO);
    }
}
