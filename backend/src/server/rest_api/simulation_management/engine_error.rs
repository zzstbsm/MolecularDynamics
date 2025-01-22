use axum::{http::StatusCode, response::{IntoResponse, Response}};

#[derive(Debug)]
pub enum EngineError {
    LatticeType,
    Integrator,
}

impl IntoResponse for EngineError{
    fn into_response(self) -> Response {
        println!("->> {:12} - {self:?}", "INTO_RES");

        match self {
            Self::LatticeType => {
                (StatusCode::BAD_REQUEST, "Inserted lattice_type not valid").into_response()
            }
            Self::Integrator => {
                (StatusCode::BAD_REQUEST, "Inserted integrator not valid").into_response()
            }
        }
    }
}

