use super::physics::{atom::Atom, dynamics::DynamicsType};

pub mod verlet;
pub mod runge_kutta;

pub trait Integrator {
    fn dynamics(
        &self,
        differential_equation_system: DynamicsType,
        atoms: &mut Vec<Atom>,
        t: f64,
        dt: f64,
        box_length: &f64,
    );
}

