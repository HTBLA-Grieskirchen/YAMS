pub struct Migration;

use async_trait::async_trait;
use molting::UpMigration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        2
    }

    fn description(&self) -> Option<&'static str> {
        Some("Deutsches Schema (klienten, haustiere, abrechnung, UTF-8)")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                "
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
                \"straße_und_hausnummer\" TEXT NOT NULL,
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
                mwst_prozentsatz TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE behandlungen (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                beschreibung TEXT NOT NULL,
                standardpreis TEXT NOT NULL,
                mwst_prozentsatz TEXT NOT NULL,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE rechnungen (
                id TEXT PRIMARY KEY,
                rechnungsnummer INTEGER NOT NULL UNIQUE,
                klient_id TEXT NOT NULL,
                rechnungsdatum TEXT NOT NULL,
                gesamtbetrag TEXT NOT NULL,
                status TEXT NOT NULL,
                bezahlt_datum TEXT,
                _version INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (klient_id) REFERENCES klienten(id)
            );

            CREATE TABLE leistungen (
                id TEXT PRIMARY KEY,
                klient_id TEXT NOT NULL,
                haustier_id TEXT,
                beschreibung TEXT NOT NULL,
                leistungsdatum TEXT NOT NULL,
                status TEXT NOT NULL,
                quelle_typ TEXT NOT NULL,
                quelle_id TEXT,
                quelle_menge TEXT,
                quelle_einzelpreis TEXT,
                quelle_preis TEXT,
                quelle_mwst_prozentsatz TEXT NOT NULL,
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
                einzelpreis TEXT NOT NULL,
                \"stückzahl\" TEXT NOT NULL,
                mwst_prozentsatz TEXT NOT NULL,
                FOREIGN KEY (rechnung_id) REFERENCES rechnungen(id),
                FOREIGN KEY (leistung_id) REFERENCES leistungen(id)
            );
        ",
            )
            .await?;

        let has_clients = table_exists(transaction, "clients").await?;
        if has_clients {
            transaction
                .execute(
                    "INSERT INTO klienten (id, vorname, nachname, geburtstag, email, mobilnummer, kundennummer, einwilligung, postleitzahl, stadt, \"straße_und_hausnummer\", \"ländercode\", _version) SELECT id, first_name, last_name, birthdate, email, mobile_number, customer_number, consent, postal_code, city, street_and_number, country_code, _version FROM clients",
                    (),
                )
                .await?;
            transaction.execute("DROP TABLE clients", ()).await?;
        }

        let has_animals = table_exists(transaction, "animals").await?;
        if has_animals {
            transaction
                .execute(
                    "INSERT INTO haustiere (id, name, geburtstag, tierart, beschreibung, klient_id, _version) SELECT id, name, birthdate, animal_species, description, client_id, _version FROM animals WHERE client_id IS NOT NULL",
                    (),
                )
                .await?;
            transaction.execute("DROP TABLE animals", ()).await?;
        }

        Ok(())
    }
}

async fn table_exists(
    transaction: &mut libsql::Transaction,
    name: &str,
) -> Result<bool, libsql::Error> {
    let mut rows = transaction
        .query(
            "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1",
            [name],
        )
        .await?;

    Ok(rows.next().await?.is_some())
}
