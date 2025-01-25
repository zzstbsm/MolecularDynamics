use axum::{extract::{Path, Query}, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use super::TAG_TEST;

pub fn api_test_route_hello() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(handler_hello))
        .routes(routes!(handler_hello2))
}

#[utoipa::path(
    get,
    path = "/hello",
    tag = TAG_TEST,
    responses(
        (status = 200, description = "Say hello", body = [HelloParams])
    )
)]
async fn handler_hello(Query(param): Query<HelloParams>) -> Json<Message> {
    println!("->> {:<12} - handler_hello", "HANDLER");
    let name = param.name.as_deref().unwrap_or("Server!");
    Json(Message {
        message: String::from(format!("Hello {name}"))
    })
}

#[utoipa::path(
    get,
    path = "/hello2/{name}",
    tag = TAG_TEST,
    responses(
        (status = 200, description = "Say hello")
    )
)]
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

#[derive(Debug, Deserialize, ToSchema)]
struct HelloParams {
    name: Option<String>
}
