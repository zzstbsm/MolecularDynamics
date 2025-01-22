use axum::{response::IntoResponse, Json};
use serde_json::{json, Value};

use crate::server::{rest_api::simulation_management::engine_payload::FormatEngine, server_error::CustomJson};

use super::engine_payload::EnginePayload;

/// Set parameters for a new run
pub async fn api_engine_set_parameters(
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
