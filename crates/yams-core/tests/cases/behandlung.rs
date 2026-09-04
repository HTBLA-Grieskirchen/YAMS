use rust_decimal::Decimal;
use yams_core::domain::{Preis, Ratio};
use yams_core::service::BehandlungErstellen;

use super::super::base_app_builder;

fn mwst_19() -> Ratio {
    Ratio::new(Decimal::new(19, 2)).unwrap()
}

#[test_log::test(pollster::test)]
async fn test_behandlung_erstellen() {
    let app = base_app_builder().await.build();

    let behandlung = app
        .execute(BehandlungErstellen {
            name: "Untersuchung".into(),
            beschreibung: "Allgemeine Untersuchung".into(),
            standardpreis: Preis::new(Decimal::new(50, 0)).unwrap(),
            mwst: mwst_19(),
        })
        .await
        .unwrap();

    assert_eq!(behandlung.name(), "Untersuchung");
    assert_eq!(behandlung.mwst().value(), Decimal::new(19, 2));
}

#[test_log::test(pollster::test)]
async fn test_behandlung_erstellen_rejects_empty_name() {
    let app = base_app_builder().await.build();

    let err = app
        .execute(BehandlungErstellen {
            name: "".into(),
            beschreibung: "Allgemeine Untersuchung".into(),
            standardpreis: Preis::new(Decimal::new(50, 0)).unwrap(),
            mwst: mwst_19(),
        })
        .await;

    assert!(err.is_err());
}
