use crate::engine::{integrators::{runge_kutta::RungeKutta, verlet::Verlet, Integrator}, physics::ensemble::Ensemble};

use super::{Engine, LatticeType, SupportedIntegrator};

impl Engine {
    pub fn new(
        number_of_atoms: u64,
        box_length: f64,
        current_time: f64,
        integration_step: f64,
        target_temperature: f64,
        lattice_type: LatticeType,
        integrator: SupportedIntegrator,
    ) -> Engine {

        let ensemble = Box::new(
            Ensemble::new(
                number_of_atoms,
                box_length,
                current_time,
                integration_step,
                target_temperature,
                lattice_type,
            )
        );
        let chosen_integrator = Self::match_integrator(integrator);

        return Engine {
            ensemble,
            integrator: chosen_integrator,
        }
    }

    pub fn load(
        run_name: &String,
        integrator: SupportedIntegrator,
    ) -> Self {

        let ensemble = Self::read_ensemble(run_name).unwrap();
        let chosen_integrator = Self::match_integrator(integrator);

        return Engine {
            ensemble: Box::new(ensemble),
            integrator: chosen_integrator,
        }       

    }

    fn match_integrator(
        integrator: SupportedIntegrator
    ) -> Box<dyn Integrator> {
        return match integrator {
            SupportedIntegrator::Verlet => Box::new(Verlet {}),
            SupportedIntegrator::RungeKutta => Box::new(RungeKutta {}),
        }
    }
}
