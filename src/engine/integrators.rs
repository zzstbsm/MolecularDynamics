use super::physics::atom::Atom;

pub mod verlet;
pub mod runge_kutta;

#[derive(Clone,clap::ValueEnum)]
pub enum SupportedIntegrator {
    Verlet,
    RungeKutta,
}

pub type IntegratorFnType = fn(
    differential_equation_system: fn(&mut Vec<Atom>,&Vec<Atom>,f64,f64),
    atoms: &mut Vec<Atom>,
    t: f64,
    dt: f64,
    box_length: f64,
);
