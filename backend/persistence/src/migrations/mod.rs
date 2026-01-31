pub mod m0001_initial;

use libsql::{Connection, params};
use yams_core::service::errors::PersistenceError as Error;

pub struct Migration {
    pub id: i32,
    pub name: &'static str,
    pub up: &'static str,
    pub down: &'static str,
}

pub fn get_migrations() -> Vec<Migration> {
    vec![Migration {
        id: 1,
        name: "initial_schema",
        up: m0001_initial::UP,
        down: m0001_initial::DOWN,
    }]
}

pub async fn init_migrations_table(conn: &Connection) -> Result<(), Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS _migrations (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        (),
    )
    .await
    .map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?;
    Ok(())
}

pub async fn migrate_up(conn: &Connection) -> Result<(), Error> {
    init_migrations_table(conn).await?;

    let migrations = get_migrations();
    
    let applied_migrations: Vec<i32> = {
        let mut rows = conn
            .query("SELECT id FROM _migrations ORDER BY id ASC", ())
            .await
            .map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?;
        
        let mut ids = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| Error::Unknown(anyhow::anyhow!(e)))? {
            ids.push(row.get(0).map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?);
        }
        ids
    };

    for m in migrations {
        if !applied_migrations.contains(&m.id) {
            println!("Applying migration {}: {}", m.id, m.name);
            conn.execute_batch(m.up)
                .await
                .map_err(|e| Error::Unknown(anyhow::anyhow!("Failed to apply migration {}: {}", m.name, e)))?;
            
            conn.execute(
                "INSERT INTO _migrations (id, name) VALUES (?1, ?2)",
                params![m.id, m.name],
            )
            .await
            .map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?;
        }
    }

    Ok(())
}

pub async fn migrate_down(conn: &Connection) -> Result<(), Error> {
    init_migrations_table(conn).await?;

    let migrations = get_migrations();
    
    let last_applied_id: Option<i32> = {
        let mut rows = conn
            .query("SELECT id FROM _migrations ORDER BY id DESC LIMIT 1", ())
            .await
            .map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?;
        
        if let Some(row) = rows.next().await.map_err(|e| Error::Unknown(anyhow::anyhow!(e)))? {
            Some(row.get(0).map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?)
        } else {
            None
        }
    };

    if let Some(id) = last_applied_id {
        if let Some(m) = migrations.iter().find(|m| m.id == id) {
            println!("Rolling back migration {}: {}", m.id, m.name);
            conn.execute_batch(m.down)
                .await
                .map_err(|e| Error::Unknown(anyhow::anyhow!("Failed to rollback migration {}: {}", m.name, e)))?;
            
            conn.execute("DELETE FROM _migrations WHERE id = ?1", params![m.id])
                .await
                .map_err(|e| Error::Unknown(anyhow::anyhow!(e)))?;
        }
    } else {
        println!("No migrations to rollback");
    }

    Ok(())
}
