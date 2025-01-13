use super::{integrators::Integrator, physics::ensemble::Ensemble};

pub mod engine_get_data;
pub mod engine_init;
pub mod engine_io;
pub mod engine_run;

pub struct Engine {
    ensemble: Box<Ensemble>,
    integrator: Box<dyn Integrator>,
}

#[derive(Clone, clap::ValueEnum)]
pub enum LatticeType {
    FCC,
    SimpleCubic,
}

#[derive(Clone, clap::ValueEnum)]
pub enum SupportedIntegrator {
    Verlet,
    RungeKutta,
}

