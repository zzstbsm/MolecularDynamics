pub mod runge_kutta;
pub mod verlet;

use super::physics::atom::Atom;

pub const DEFAULT_INTEGRATOR: SupportedIntegrator = SupportedIntegrator::Verlet;

pub enum SupportedIntegrator {
    Verlet,
    RungeKutta,
}

pub fn select_integrator(
    integrator: SupportedIntegrator
) -> fn(
    fn(&mut Vec<Atom>,&Vec<Atom>,f64,f64),
    &mut Vec<Atom>,
    f64,
    f64,
    f64,
)  {
    return match integrator {
        SupportedIntegrator::Verlet => verlet::verlet,
        SupportedIntegrator::RungeKutta => runge_kutta::runge_kutta,
    };
}