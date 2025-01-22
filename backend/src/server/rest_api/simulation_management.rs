use axum::{routing::post, Router};
use feature_engine_set_parameters::api_engine_set_parameters;

mod engine_error;
mod engine_payload;

mod feature_engine_set_parameters;

// ---- Routes: Handle the simulations
pub fn api_simulation_management() -> Router {
    Router::new()
        .route("/simulation/management/set", post(api_engine_set_parameters))
}

