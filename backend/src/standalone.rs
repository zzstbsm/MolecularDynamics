use crate::{engine::{integrators::Integrator, physics::{dynamics::dynamics, ensemble::Ensemble}}, io::MDIO, run_instance::RunInstance};

pub struct Standalone {
    pub ensemble: Ensemble,
    pub chosen_integrator: Box<dyn Integrator>,
    pub run_name: String,
}

impl RunInstance for Standalone {
    fn start(&mut self){

        MDIO::write_ensemble(&self.run_name, &self.ensemble);

        let properties_preamble = MDIO::get_properties(&self.ensemble, true);
        print!("{}",properties_preamble);
        let properties_string = MDIO::write_properties(&self.run_name, &self.ensemble, true);
        print!("{}",properties_string);

        while self.ensemble.t < 1e3 {
            
            self.ensemble.run_step(
                &(*self.chosen_integrator),
                dynamics,
                1000
            );
            
            MDIO::write_ensemble(&self.run_name, &self.ensemble);
            let properties_string = MDIO::write_properties(&self.run_name, &self.ensemble, false);
            print!("{}",properties_string);

        }
    }
}
