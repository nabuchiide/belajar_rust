use dotenv::dotenv;
use envconfig::Envconfig;

use crate::constant::app_settings::AppSettings;
use crate::config::db_conf::main_db_connection;
use crate::routes::start_server::server_start;

mod constant;
mod config;
mod routes;
mod payload;
mod domain;
mod services;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let app_settings = AppSettings::init_from_env().unwrap();
    let pool = main_db_connection(app_settings.database_url())
        .await.expect("Failed to create database pool");
    server_start(pool).await;
}

