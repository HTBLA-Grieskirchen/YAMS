pub struct Migration;

use async_trait::async_trait;
use molting::UpMigration;

#[async_trait]
impl UpMigration<libsql::Transaction, libsql::Error> for Migration {
    fn version(&self) -> usize {
        5
    }

    fn description(&self) -> Option<&'static str> {
        Some("Rename leftover *_prozentsatz MwSt columns to match the repository")
    }

    async fn up(&self, transaction: &mut libsql::Transaction) -> Result<(), libsql::Error> {
        align_column(
            transaction,
            "leistungen",
            "quelle_mwst_prozentsatz",
            "quelle_mwst",
        )
        .await?;
        align_column(transaction, "produkte", "mwst_prozentsatz", "mwst").await?;
        align_column(transaction, "behandlungen", "mwst_prozentsatz", "mwst").await?;
        align_column(
            transaction,
            "rechnungspositionen",
            "mwst_prozentsatz",
            "mwst",
        )
        .await?;
        Ok(())
    }
}

async fn align_column(
    transaction: &mut libsql::Transaction,
    table: &str,
    old: &str,
    new: &str,
) -> Result<(), libsql::Error> {
    let has_old = column_exists(transaction, table, old).await?;
    let has_new = column_exists(transaction, table, new).await?;
    match (has_old, has_new) {
        (true, false) => {
            transaction
                .execute(
                    &format!("ALTER TABLE {table} RENAME COLUMN {old} TO {new}"),
                    (),
                )
                .await?;
        }
        (true, true) => {
            transaction
                .execute(&format!("UPDATE {table} SET {new} = {old}"), ())
                .await?;
            transaction
                .execute(&format!("ALTER TABLE {table} DROP COLUMN {old}"), ())
                .await?;
        }
        (false, true) | (false, false) => {}
    }
    Ok(())
}

async fn column_exists(
    transaction: &mut libsql::Transaction,
    table: &str,
    column: &str,
) -> Result<bool, libsql::Error> {
    let mut rows = transaction
        .query(&format!("PRAGMA table_info({table})"), ())
        .await?;
    while let Some(row) = rows.next().await? {
        let name: String = row.get(1)?;
        if name == column {
            return Ok(true);
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn columns(conn: &libsql::Connection, table: &str) -> Vec<String> {
        let mut rows = conn
            .query(&format!("PRAGMA table_info({table})"), ())
            .await
            .unwrap();
        let mut names = Vec::new();
        while let Some(row) = rows.next().await.unwrap() {
            names.push(row.get::<String>(1).unwrap());
        }
        names
    }

    async fn apply(conn: &libsql::Connection) {
        let mut tx = conn.transaction().await.unwrap();
        Migration.up(&mut tx).await.unwrap();
        tx.commit().await.unwrap();
    }

    async fn memory() -> libsql::Connection {
        let db = libsql::Builder::new_local(":memory:")
            .build()
            .await
            .unwrap();
        db.connect().unwrap()
    }

    #[pollster::test]
    async fn merge_drops_quelle_mwst_prozentsatz_so_insert_matches_repo() {
        let conn = memory().await;
        conn.execute_batch(
            "
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
                quelle_mwst TEXT NOT NULL DEFAULT '0'
            );
            ",
        )
        .await
        .unwrap();

        apply(&conn).await;

        let names = columns(&conn, "leistungen").await;
        assert!(names.contains(&"quelle_mwst".into()));
        assert!(!names.contains(&"quelle_mwst_prozentsatz".into()));

        conn.execute(
            "INSERT INTO leistungen (id, klient_id, haustier_id, beschreibung, leistungsdatum, status, quelle_typ, quelle_id, quelle_menge, quelle_einzelpreis, quelle_preis, quelle_mwst, rechnung_id, _version) VALUES (?1, ?2, ?3, ?4, ?5, 'offen', ?6, ?7, ?8, ?9, ?10, ?11, NULL, ?12)",
            libsql::params![
                "id",
                "klient",
                Option::<String>::None,
                "Seminar",
                "2026-08-25",
                "seminar",
                Option::<String>::None,
                Option::<String>::None,
                Option::<String>::None,
                Option::<String>::None,
                "0.20",
                0u64,
            ],
        )
        .await
        .expect("repo INSERT must not hit quelle_mwst_prozentsatz NOT NULL");
    }

    #[pollster::test]
    async fn rename_only_prozentsatz_column_when_new_name_absent() {
        let conn = memory().await;
        conn.execute(
            "CREATE TABLE produkte (id TEXT PRIMARY KEY, mwst_prozentsatz TEXT NOT NULL)",
            (),
        )
        .await
        .unwrap();

        apply(&conn).await;

        let names = columns(&conn, "produkte").await;
        assert!(names.contains(&"mwst".into()));
        assert!(!names.contains(&"mwst_prozentsatz".into()));
    }

    #[pollster::test]
    async fn already_aligned_schema_is_noop() {
        let conn = memory().await;
        conn.execute(
            "CREATE TABLE leistungen (id TEXT PRIMARY KEY, quelle_mwst TEXT NOT NULL)",
            (),
        )
        .await
        .unwrap();

        apply(&conn).await;

        let names = columns(&conn, "leistungen").await;
        assert_eq!(names.iter().filter(|n| n.contains("mwst")).count(), 1);
        assert!(names.contains(&"quelle_mwst".into()));
    }
}
