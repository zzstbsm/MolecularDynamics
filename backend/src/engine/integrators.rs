use super::physics::{atom::Atom, dynamics::DynamicsType};

pub mod verlet;
pub mod runge_kutta;

pub trait Integrator {
    fn dynamics(
        &self,
        differential_equation_system: DynamicsType,
        atoms: &mut Vec<Atom>,
        current_time: f64,
        integration_step: f64,
        box_length: &f64,
    );
}

