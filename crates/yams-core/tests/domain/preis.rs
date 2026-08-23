use rust_decimal::Decimal;
use yams_core::domain::Preis;

#[test]
fn preis_zero_is_zero() {
    assert_eq!(Preis::zero().value(), Decimal::ZERO);
}

#[test]
fn preis_add_sums_values() {
    let a = Preis::new(Decimal::new(1250, 2)).unwrap();
    let b = Preis::new(Decimal::new(750, 2)).unwrap();

    assert_eq!((a + b).value(), Decimal::new(20, 0));
}

#[test]
fn preis_multiply_scales_value() {
    let preis = Preis::new(Decimal::new(25, 0)).unwrap();

    assert_eq!(
        preis.multiply(Decimal::new(2, 0)).unwrap().value(),
        Decimal::new(50, 0)
    );
}

#[test]
fn preis_new_rejects_negative() {
    assert!(Preis::new(Decimal::new(-1, 0)).is_err());
}
