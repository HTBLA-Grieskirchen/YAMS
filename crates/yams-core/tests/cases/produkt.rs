use rust_decimal::Decimal;
use yams_core::domain::{Preis, Ratio};
use yams_core::service::ProduktErstellen;

use super::super::base_app_builder;

fn mwst_19() -> Ratio {
    Ratio::new(Decimal::new(19, 2)).unwrap()
}

#[test_log::test(pollster::test)]
async fn test_produkt_erstellen() {
    let app = base_app_builder().await.build();

    let produkt = app
        .execute(ProduktErstellen {
            name: "Futter".into(),
            beschreibung: "Premium Futter".into(),
            einzelpreis: Preis::new(Decimal::new(25, 0)).unwrap(),
            mwst: mwst_19(),
        })
        .await
        .unwrap();

    assert_eq!(produkt.name(), "Futter");
    assert_eq!(produkt.mwst().value(), Decimal::new(19, 2));
}

#[test_log::test(pollster::test)]
async fn test_produkt_erstellen_rejects_empty_name() {
    let app = base_app_builder().await.build();

    let err = app
        .execute(ProduktErstellen {
            name: "".into(),
            beschreibung: "Premium Futter".into(),
            einzelpreis: Preis::new(Decimal::new(25, 0)).unwrap(),
            mwst: mwst_19(),
        })
        .await;

    assert!(err.is_err());
}
