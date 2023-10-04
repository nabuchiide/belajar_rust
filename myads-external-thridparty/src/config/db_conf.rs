use sqlx::{MySqlPool};

use sqlx::mysql::MySqlPoolOptions;

pub struct MySQLMainDBPooling {
    pub db: MySqlPool,
}

pub async fn main_db_connection(uri_db: &str) -> Result<MySqlPool, sqlx::Error> {
    let databse_url = uri_db;
    let pool = MySqlPoolOptions::new()
        .max_connections(10).connect(&databse_url).await?;
    println!("✅ Connection to the database is successful!");
    Ok(pool)
}