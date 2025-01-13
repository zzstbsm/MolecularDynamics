use std::net::SocketAddr;

use axum::{
    routing::get_service, Router
};
use tower_http::services::ServeDir;

use super::rest_api;

#[tokio::main]
pub async fn server_main(port: u16){
    let app: Router = Router::new()
        .merge(rest_api::test_routes::api_test_routes())
        .merge(rest_api::simulation_management::api_simulation_management())
        .merge(rest_api::simulation_results::api_simulation_results())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Server started, listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");

}

// ---- Routes: Hello static files routes
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

