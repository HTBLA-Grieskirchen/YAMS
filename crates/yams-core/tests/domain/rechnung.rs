use rust_decimal::Decimal;
use uuid::Uuid;
use yams_core::domain::{LeistungId, Preis, Rechnungsposition};

#[test]
fn rechnungsposition_berechnet_mwst_korrekt() {
    let position = Rechnungsposition::neu(
        "Untersuchung".into(),
        Preis::new(Decimal::new(100, 0)).unwrap(),
        Decimal::ONE,
        Decimal::new(19, 0),
        LeistungId(Uuid::new_v4()),
    );

    assert_eq!(position.gesamtpreis_netto().value(), Decimal::new(100, 0));
    assert_eq!(position.mwst_betrag().value(), Decimal::new(19, 0));
    assert_eq!(position.gesamtpreis_brutto().value(), Decimal::new(119, 0));
}

#[test]
fn rechnungsposition_mit_stueckzahl_multipliziert_netto() {
    let position = Rechnungsposition::neu(
        "Futter".into(),
        Preis::new(Decimal::new(25, 0)).unwrap(),
        Decimal::new(2, 0),
        Decimal::new(19, 0),
        LeistungId(Uuid::new_v4()),
    );

    assert_eq!(position.gesamtpreis_netto().value(), Decimal::new(50, 0));
    assert_eq!(position.mwst_betrag().value(), Decimal::new(95, 1));
    assert_eq!(position.gesamtpreis_brutto().value(), Decimal::new(595, 1));
}
