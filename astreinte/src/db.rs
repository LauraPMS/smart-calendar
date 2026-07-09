// Ce fichier va contenir la database (d'ou db)

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn init_pool(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
        
    Ok(pool)
}