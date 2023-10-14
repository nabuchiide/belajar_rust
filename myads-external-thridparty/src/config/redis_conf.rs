use std::str::Split;
use envconfig::Envconfig;
use redis::AsyncCommands;
use redis::cluster::{ClusterClient, ClusterConnection};

use crate::constant::app_settings::AppSettings;

pub async fn redis_cluster_connect() -> redis::cluster_async::ClusterConnection {
    let app_settings = AppSettings::init_from_env().unwrap();
    let redis_url = app_settings.redis_cluster_node();
    let nodes:Vec<&str> = redis_url.split(",").collect();
    let client = ClusterClient::new(nodes).unwrap();
    let mut connection = client.get_async_connection().await.unwrap();
    let _: () = connection.set("test", "test_data").await.unwrap();
    let rv: String = connection.get("test").await.unwrap();
    return  connection;
}

