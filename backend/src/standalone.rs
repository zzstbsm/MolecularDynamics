use crate::{engine::api::Engine, run_instance::RunInstance};

pub struct Standalone {
    pub engine: Box<Engine>,
    pub run_name: String,
}

impl RunInstance for Standalone {

    fn start(&mut self){

        let mut t = 0_f64;
        let t_max = 1e3_f64;

        self.engine.write_ensemble(&self.run_name);

        let properties_preamble = self.engine.get_properties(true);
        print!("{}",properties_preamble);
        let properties_string = self.engine.write_properties(&self.run_name,true);
        print!("{}",properties_string);

        while t < t_max {
        // while self.ensemble.t < 1e3 {
            self.engine.run(
                1000
            );
            
            self.engine.write_ensemble(&self.run_name);
            let properties_string = self.engine.write_properties(&self.run_name, true);
            print!("{}",properties_string);
            
            t += self.engine.get_integration_step();
        }
    }
}
