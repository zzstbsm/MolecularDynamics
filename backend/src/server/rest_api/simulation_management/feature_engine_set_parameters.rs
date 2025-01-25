use axum::{response::IntoResponse, Json};
use serde_json::{json, Value};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::server::{rest_api::simulation_management::engine_payload::FormatEngine, server_error::CustomJson};

use super::{engine_error::EngineError, engine_payload::EnginePayload, TAG_SIMULATION_MANAGEMENT};

// ---- Routes: Handle the simulations
pub fn feature_engine_set_parameters() -> OpenApiRouter {
    return OpenApiRouter::new()
        .routes(routes!(api_engine_set_parameters));
}

/// Set parameters for a new run
#[utoipa::path(
    post,
    path = "/simulation/management/set",
    tag = TAG_SIMULATION_MANAGEMENT,
    responses(
        (status = 200, description = "Engine set successfully", body = EnginePayload),
        (status = 404, description = "Error in the paylod parameters", body = EngineError)
    )
)]
async fn api_engine_set_parameters(
    CustomJson(payload): CustomJson<EnginePayload>
) -> Result<Json<Value>, impl IntoResponse> {

    println!("->> {:12} - api_engine_set_parameters", "HANDLER");

    let engine = payload.format();
    return match engine {
        Ok(_v) => {
            // TODO implement logic to start a new process with the engine
            let body = Json(json!({
                "result" : {
                    "success": true
                }
            }));
            Ok(body)
        }
        Err(e) => {
            return Err(e);
        }
    }

}
