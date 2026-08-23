use std::ops::Add;

use rust_decimal::Decimal;
use thiserror::Error;

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

    pub fn multiply(&self, factor: Decimal) -> Result<Self, PreisFehler> {
        Self::new(self.0 * factor)
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
