use libsql::Connection;
use rust_embed::RustEmbed;
use std::fs;
use tempfile::TempDir;

#[derive(RustEmbed)]
#[folder = "migrations/"]
struct Migrations;

pub async fn run_migrations(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = TempDir::new()?;
    let tmp_path = tmp_dir.path();

    // Extract embedded migrations to temp directory
    for file in Migrations::iter() {
        let content = Migrations::get(&file).ok_or("Failed to get embedded migration")?;
        let path = tmp_path.join(file.as_ref());

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, content.data)?;
        println!("Wrote migration file: {}", fs::read_to_string(path)?);
    }

    // Apply migrations using libsql_migration
    libsql_migration::dir::migrate(conn, tmp_path.to_path_buf())
        .await
        .map_err(|e| format!("Migration failed: {}", e))?;

    Ok(())
}
