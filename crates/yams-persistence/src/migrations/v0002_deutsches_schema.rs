pub struct Migration;

use async_trait::async_trait;
use molting::UpMigration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        2
    }

    fn description(&self) -> Option<&'static str> {
        Some("Deutsches Schema (klienten, haustiere, abrechnung)")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                "
            DROP TABLE IF EXISTS animals;
            DROP TABLE IF EXISTS clients;

            CREATE TABLE klienten (
                id TEXT PRIMARY KEY,
                vorname TEXT NOT NULL,
                nachname TEXT NOT NULL,
                geburtstag TEXT NOT NULL,
                email TEXT NOT NULL,
                mobilnummer TEXT NOT NULL,
                kundennummer INTEGER NOT NULL UNIQUE,
                einwilligung INTEGER NOT NULL,
                postleitzahl TEXT NOT NULL,
                stadt TEXT NOT NULL,
                strasse_und_hausnummer TEXT NOT NULL,
                \"ländercode\" TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE haustiere (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                geburtstag TEXT NOT NULL,
                tierart TEXT NOT NULL,
                beschreibung TEXT NOT NULL,
                klient_id TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (klient_id) REFERENCES klienten(id)
            );

            CREATE TABLE produkte (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                beschreibung TEXT NOT NULL,
                einzelpreis TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE behandlungen (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                beschreibung TEXT NOT NULL,
                standardpreis TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE rechnungen (
                id TEXT PRIMARY KEY,
                rechnungsnummer INTEGER NOT NULL UNIQUE,
                klient_id TEXT NOT NULL,
                rechnungsdatum TEXT NOT NULL,
                gesamtbetrag TEXT NOT NULL,
                status TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (klient_id) REFERENCES klienten(id)
            );

            CREATE TABLE leistungen (
                id TEXT PRIMARY KEY,
                klient_id TEXT NOT NULL,
                haustier_id TEXT,
                beschreibung TEXT NOT NULL,
                betrag TEXT NOT NULL,
                leistungsdatum TEXT NOT NULL,
                status TEXT NOT NULL,
                quelle_typ TEXT NOT NULL,
                quelle_id TEXT,
                rechnung_id TEXT,
                _version INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (klient_id) REFERENCES klienten(id),
                FOREIGN KEY (haustier_id) REFERENCES haustiere(id),
                FOREIGN KEY (rechnung_id) REFERENCES rechnungen(id)
            );

            CREATE TABLE rechnungspositionen (
                id TEXT PRIMARY KEY,
                rechnung_id TEXT NOT NULL,
                leistung_id TEXT NOT NULL,
                beschreibung TEXT NOT NULL,
                betrag TEXT NOT NULL,
                FOREIGN KEY (rechnung_id) REFERENCES rechnungen(id),
                FOREIGN KEY (leistung_id) REFERENCES leistungen(id)
            );
        ",
            )
            .await?;
        Ok(())
    }
}
