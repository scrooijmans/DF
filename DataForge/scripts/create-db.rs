// Simple script to create the database with schema
use dataforge_core::db;

fn main() -> anyhow::Result<()> {
    let db_path = std::path::Path::new(
        &std::env::var("HOME").unwrap()
    ).join("Library/Application Support/com.dataforge.app/dataforge.db");
    
    println!("Creating database at: {:?}", db_path);
    
    // Create directory if it doesn't exist
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    // Open/create database (this will initialize the schema)
    let _conn = db::open_db(&db_path)?;
    
    println!("✅ Database created successfully!");
    println!("You can now open it with:");
    println!("  open -a \"DB Browser for SQLite\" {:?}", db_path);
    
    Ok(())
}

