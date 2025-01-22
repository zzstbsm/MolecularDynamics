use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::server::server_error::ServerError;

pub fn api_test_routes_login() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>, impl IntoResponse> {

    println!("->> {:12} - api_login", "HANDLER");

    // TODO: Implement authentication logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(ServerError::LoginFail);
    }

    // TODO: Set cookies

    // Create the successful body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String
}
