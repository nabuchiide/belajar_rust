use std::net::SocketAddr;
use std::sync::Arc;
use axum::response::IntoResponse;
use axum::{Json, Router, Server};
use axum::routing::get;
use envconfig::Envconfig;
use sqlx::MySqlPool;
use crate::constant::api_constant::PATH_API_BASE;
use crate::config::db_conf::MySQLMainDBPooling;
use crate::constant::app_settings::AppSettings;

pub async fn server_start(pool: MySqlPool) {
    let app_settings = AppSettings::init_from_env().unwrap();
    let addr = SocketAddr::from(([0, 0, 0, 0], app_settings.server_port()));
    println!("🚀 Server started successfully");
    println!("server port :: {}", &addr);
    let route = route_api(pool);
    Server::bind(&addr).serve(route.into_make_service()).await.unwrap();
}

fn route_api(pool: MySqlPool) -> Router {
    println!("{}", &PATH_API_BASE);
    Router::new().route(PATH_API_BASE, get(health_checker_handler))
        .with_state(Arc::new(MySQLMainDBPooling { db: pool.clone() }))
}

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Rust CRUD API Example with Axum Framework and MySQL";

    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

    Json(json_response)
}