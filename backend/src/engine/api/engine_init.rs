use crate::engine::{integrators::{runge_kutta::RungeKutta, verlet::Verlet, Integrator}, physics::ensemble::Ensemble};

use super::{Engine, LatticeType, SupportedIntegrator};

impl Engine {
    pub fn new(
        number_of_atoms: u64,
        box_length: f64,
        t: f64,
        dt: f64,
        target_temperature: f64,
        lattice_type: LatticeType,
        integrator: SupportedIntegrator,
    ) -> Engine {

        let ensemble = Box::new(
            Ensemble::new(
                number_of_atoms, //200_u64,
                box_length,
                t,
                dt,
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
