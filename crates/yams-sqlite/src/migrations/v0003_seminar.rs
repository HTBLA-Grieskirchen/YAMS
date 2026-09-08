pub struct Migration;

use async_trait::async_trait;
use molting::UpMigration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        3
    }

    fn description(&self) -> Option<&'static str> {
        Some("Seminare, Termine und Buchungen")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                "
            CREATE TABLE seminare (
                id TEXT PRIMARY KEY,
                titel TEXT NOT NULL,
                beschreibung TEXT NOT NULL,
                \"teilnahmegebühr_basis\" TEXT NOT NULL,
                mwst TEXT NOT NULL,
                standarddauer_ms INTEGER,
                _version INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE seminar_termine (
                id TEXT PRIMARY KEY,
                seminar_id TEXT NOT NULL,
                beginn TEXT NOT NULL,
                ende TEXT NOT NULL,
                ort_name TEXT,
                postleitzahl TEXT,
                stadt TEXT,
                \"straße_und_hausnummer\" TEXT,
                \"ländercode\" TEXT,
                max_teilnehmer INTEGER,
                status TEXT NOT NULL,
                abgehalten_am TEXT,
                abgesagt_am TEXT,
                absagegrund TEXT,
                _version INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (seminar_id) REFERENCES seminare(id)
            );

            CREATE TABLE seminar_buchungen (
                id TEXT PRIMARY KEY,
                termin_id TEXT NOT NULL,
                klient_id TEXT NOT NULL,
                rabatt TEXT NOT NULL,
                storniert_am TEXT,
                leistung_id TEXT,
                FOREIGN KEY (termin_id) REFERENCES seminar_termine(id),
                FOREIGN KEY (klient_id) REFERENCES klienten(id),
                FOREIGN KEY (leistung_id) REFERENCES leistungen(id)
            );
            ",
            )
            .await?;
        Ok(())
    }
}
