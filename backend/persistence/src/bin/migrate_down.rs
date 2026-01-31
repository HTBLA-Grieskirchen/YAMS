use yams_persistence::migrations::migrate_down;
use libsql::Builder;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <db_path>", args[0]);
        std::process::exit(1);
    }
    let db_path = &args[1];
    
    let db = Builder::new_local(db_path).build().await?;
    let conn = db.connect()?;
    
    migrate_down(&conn).await?;
    println!("Migration rolled back successfully (down)");
    Ok(())
}
