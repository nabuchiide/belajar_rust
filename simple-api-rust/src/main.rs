mod routes;

#[tokio::main]
async fn main() {
    routes::routes_api().await;
}
