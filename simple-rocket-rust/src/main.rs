mod routes;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    let app_data = routes::models::AppState::init();
    rocket::build().manage(app_data).mount(
        "/api",
        routes![
            routes::handler::health_checker_handler,
            routes::handler::todos_list_handler,
            routes::handler::create_todo_handler,
            routes::handler::get_todo_handler,
            routes::handler::edit_todo_handler,
            routes::handler::delete_todo_handler
        ],
    )
}
