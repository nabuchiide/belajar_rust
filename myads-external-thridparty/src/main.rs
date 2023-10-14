use dotenv::dotenv;
use envconfig::Envconfig;
use serde_json::Value::String;

use crate::config::db_conf::main_connection;
use crate::config::redis_conf::redis_cluster_connect;
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
    let redis_conn = redis_cluster_connect().await;
    let pool = main_connection().await;
    server_start(pool).await;
}

