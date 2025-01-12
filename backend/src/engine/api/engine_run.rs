use crate::engine::physics::dynamics::dynamics;

use super::Engine;

impl Engine {
    pub fn run(
        &mut self,
        steps_to_do: u64,
    ) {
        self.ensemble.run_step(
            &(*self.integrator),
            dynamics,
            steps_to_do,
        );
    }
}

