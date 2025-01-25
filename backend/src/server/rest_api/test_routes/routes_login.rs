use axum::{response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::server::server_error::ServerError;

use super::TAG_TEST;

pub fn api_test_routes_login() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(api_login))
}

#[utoipa::path(
    post,
    path = "/api/login",
    tag = TAG_TEST,
    responses(
        (status = 201, description = "Login Successfull", body = LoginPayload),
        (status = 401, description = "Login Error")
    )
)]
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

#[derive(Debug, Deserialize, ToSchema)]
struct LoginPayload {
    username: String,
    pwd: String
}
