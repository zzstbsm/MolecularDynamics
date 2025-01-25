use feature_engine_set_parameters::feature_engine_set_parameters;
use utoipa_axum::router::OpenApiRouter;

mod engine_error;
mod engine_payload;

mod feature_engine_set_parameters;

pub const TAG_SIMULATION_MANAGEMENT: &str = "Management Simulation";

// ---- Routes: Handle the simulations
pub fn api_simulation_management() -> OpenApiRouter {
    return OpenApiRouter::new()
        .merge(feature_engine_set_parameters());
}

