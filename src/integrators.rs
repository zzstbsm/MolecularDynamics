pub mod verlet;
pub mod runge_kutta;


#[allow(dead_code)]
pub enum SupportedIntegrator {
    Verlet,
    RungeKutta,
}