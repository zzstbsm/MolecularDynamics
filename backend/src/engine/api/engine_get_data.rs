use super::Engine;

impl Engine {
    pub fn get_integration_step(&self) -> f64 {
        return self.ensemble.integration_step;
    }
}
