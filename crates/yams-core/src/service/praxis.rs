use crate::domain::Ländercode;
use crate::ports::PraxisAngaben;

/// Hardcoded practitioner identity for generated documents. Not a domain aggregate.
pub fn praxis() -> PraxisAngaben {
    PraxisAngaben {
        name: "Energetik Sabine Petschl".into(),
        straße_und_hausnummer: "Hauptstraße 12".into(),
        postleitzahl: "4710".into(),
        stadt: "Grieskirchen".into(),
        ländercode: Ländercode::AT,
        email: "sabine@energetik-petschl.at".into(),
        telefon: "+437242123456".into(),
        uid: "ATU00000000".into(),
    }
}
