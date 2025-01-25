use axum::response::IntoResponse;
use serde::Deserialize;
use utoipa::ToSchema;

use crate::engine::api::{Engine, LatticeType, SupportedIntegrator};

use super::engine_error::EngineError;

#[derive(Debug, Deserialize, ToSchema)]
pub struct EnginePayload {
    number_of_atoms: u64,
    box_length: f64,
    current_time: f64,
    integration_step: f64,
    target_temperature: f64,
    lattice_type: String,
    integrator: String,
}

pub trait FormatEngine {
    fn format(self) -> Result<Engine, impl IntoResponse>;
}

impl FormatEngine for EnginePayload {

    /// Format the received Json into an engine struct usable to run the simulation
    fn format(self) -> Result<Engine, impl IntoResponse> {

        let lattice_type: LatticeType;
        match self.lattice_type.as_str() {
            "FCC" => {
                lattice_type = LatticeType::FCC;
            }
            "SimpleCubic" => {
                lattice_type = LatticeType::SimpleCubic;
            }
            _ => {
                return Err(EngineError::LatticeType);
            }
        };

        let integrator : SupportedIntegrator;
        match self.integrator.as_str() {
            "Verlet" => {
                integrator = SupportedIntegrator::Verlet;
            }
            "RungeKutta" => {
                integrator = SupportedIntegrator::RungeKutta;
            }
            _ => {
                return Err(EngineError::Integrator);
            }
        };

        let engine = Engine::new(
            self.number_of_atoms,
            self.box_length,
            self.current_time,
            self.integration_step,
            self.target_temperature,
            lattice_type,
            integrator,
        );

        Ok(engine)

    }
}

impl Clone for EnginePayload {
    fn clone(&self) -> Self {
        return EnginePayload {
            number_of_atoms: self.number_of_atoms,
            box_length: self.box_length,
            current_time: self.current_time,
            integration_step: self.integration_step,
            target_temperature: self.target_temperature,
            lattice_type: self.lattice_type.clone(),
            integrator: self.integrator.clone(),
        }
    }
}
