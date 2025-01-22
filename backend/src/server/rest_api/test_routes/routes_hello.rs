use axum::{extract::{Path, Query}, routing::get, Json, Router};
use serde::Deserialize;

pub fn api_test_route_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
}

async fn handler_hello(Query(param): Query<HelloParams>) -> Json<Message> {
    println!("->> {:<12} - handler_hello", "HANDLER");
    let name = param.name.as_deref().unwrap_or("Server!");
    Json(Message {
        message: String::from(format!("Hello {name}"))
    })
}

async fn handler_hello2(Path(param): Path<HelloParams>) -> Json<Message> {
    println!("->> {:<12} - handler_hello2", "HANDLER");
    let name = param.name.as_deref().unwrap_or("Server!");
    Json(Message {
        message: String::from(format!("Hello {name}"))
    })
}

#[derive(serde::Serialize)]
struct Message {
    message: String
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}
