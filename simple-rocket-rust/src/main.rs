use rocket::get;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

#[macro_use]
extern crate rocket;

#[derive(Serialize)]
pub struct GenerateResponse {
    pub status: String,
    pub message: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![health_checker_handler])
}

#[get("/healthchecker")]
pub async fn health_checker_handler() -> Result<Json<GenerateResponse>, Status> {
    const MESSAGE: &str = "Build Simple CRUD API With Rust and Rocket";

    let response_json = GenerateResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(Json(response_json))
}