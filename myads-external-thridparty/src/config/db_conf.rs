use envconfig::Envconfig;
use sqlx::{MySqlPool};

use sqlx::mysql::MySqlPoolOptions;
use crate::constant::app_settings::AppSettings;

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

pub async fn main_connection() -> MySqlPool {
    let app_settings = AppSettings::init_from_env().unwrap();
    let pool = main_db_connection(app_settings.database_url())
        .await.expect("Failed to create database pool");
    return pool
}