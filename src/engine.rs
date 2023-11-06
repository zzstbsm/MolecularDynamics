mod data_structure;
mod integrators;
mod physics;

mod engine_implementation;
pub mod engine_parameters;

use self::engine_parameters::RunSimulationParameters;
use self::physics::ensemble::Ensemble;


pub struct Engine {
    engine_parameters: RunSimulationParameters,
    ensemble: Ensemble,
}

impl Engine {
    pub fn new(
        run_simulation_parameters: RunSimulationParameters
    ) -> Engine {
        Engine {
            engine_parameters: run_simulation_parameters,
            ensemble: Ensemble::new(
                256_u64, //200_u64,
                20_f64,
                0_f64,
                1e-3_f64,
                1_f64,
            ),
        }
    }
}