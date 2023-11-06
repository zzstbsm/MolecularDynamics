use super::Engine;

use super::super::physics;

use physics::dynamics::dynamics as one_step;
use physics::ensemble::periodic_conditions;

impl Engine {

    pub fn run(&mut self,number_of_steps: u64) {
    
        let mut counter = 0_u64;
        while counter < number_of_steps {
    
            self.step();

            self.ensemble.t += self.ensemble.dt;
            counter += 1;
    
        }
        println!("Successful!");
    }

    fn step(&mut self) {
        (self.engine_parameters.chosen_integrator)(
            one_step,
            &mut self.ensemble.atoms,
            self.ensemble.t, 
            self.ensemble.dt,
            self.ensemble.box_length,
        );
        periodic_conditions(&mut self.ensemble);
    }
}