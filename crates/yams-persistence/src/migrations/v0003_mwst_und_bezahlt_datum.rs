pub struct Migration;

use async_trait::async_trait;
use molting::UpMigration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        3
    }

    fn description(&self) -> Option<&'static str> {
        Some("MwSt auf Katalog, Leistung-Quelle, Rechnung bezahlt_datum")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        transaction
            .execute_batch(
                "
            ALTER TABLE produkte ADD COLUMN mwst_prozentsatz TEXT NOT NULL DEFAULT '19';
            ALTER TABLE behandlungen ADD COLUMN mwst_prozentsatz TEXT NOT NULL DEFAULT '19';
            ALTER TABLE leistungen ADD COLUMN quelle_mwst_prozentsatz TEXT NOT NULL DEFAULT '19';
            ALTER TABLE rechnungen ADD COLUMN bezahlt_datum TEXT;
        ",
            )
            .await?;

        Ok(())
    }
}
