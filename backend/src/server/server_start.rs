use axum::{http::StatusCode, response::IntoResponse, Router};

use super::rest_api;

#[tokio::main]
pub async fn server_main(port: u16){
    let app: Router = Router::new()
        .merge(rest_api::test_routes::api_test_routes())
        .merge(rest_api::simulation_management::api_simulation_management())
        .merge(rest_api::simulation_results::api_simulation_results())
        .fallback(fallback_handler);

    let addr = format!("127.0.0.1:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await;
    
    match listener {
        Ok(v) => {
            axum::serve(v,app).await.unwrap();
        }
        Err(e) => {
            println!("Error {e} on server {addr}");
        }
    }

}

async fn fallback_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Endpoint not present")
}

