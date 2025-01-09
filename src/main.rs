use clap::Parser;
use engine::integrators::{verlet::Verlet,runge_kutta::RungeKutta};

mod cli;
use crate::cli::Cli;

mod engine;
use crate::engine::data_structure;
use crate::engine::integrators;
use crate::engine::integrators::Integrator;
use crate::engine::physics;
use crate::engine::physics::dynamics::dynamics;
use crate::engine::physics::ensemble::Ensemble;
use crate::engine::physics::lattice;

mod io;
use crate::io::MDIO;

fn main() {

    // Define run parameters
    let mut ensemble: Ensemble;
    let chosen_integrator: Box<dyn Integrator>;
    let run_name: String;

    let _args = Cli::parse();


    (ensemble, chosen_integrator, run_name) = apply_cli_parameters(_args);
    

    MDIO::write_ensemble(&run_name, &ensemble);

    let properties_preamble = MDIO::get_properties(&ensemble, true);
    print!("{}",properties_preamble);
    let properties_string = MDIO::write_properties(&run_name, &ensemble, true);
    print!("{}",properties_string);

    while ensemble.t < 1e3 {
        
        ensemble.run_step(
            &(*chosen_integrator),
            dynamics,
            1000
        );
        
        MDIO::write_ensemble(&run_name, &ensemble);
        let properties_string = MDIO::write_properties(&run_name, &ensemble, false);
        print!("{}",properties_string);

    }
}

/// Prova
fn apply_cli_parameters(_args: Cli) -> (Ensemble, Box<dyn Integrator>, String) {

    // Define run parameters
    let ensemble: Ensemble;
    let chosen_integrator: Box<dyn Integrator>;
    let run_name: String;

    match _args.run_type {
        cli::run_type::RunType::New { 
            name: set_name, // TODO implement run_name when saving
            n_atoms: set_atoms, 
            integrator: set_integrator, 
            boxlength: set_boxlength,
            step: set_step,
            temperature: set_temperature
        } => {
            
            run_name = set_name;

            chosen_integrator = match set_integrator {
                integrators::SupportedIntegrator::Verlet => Box::new(Verlet {}),
                integrators::SupportedIntegrator::RungeKutta => Box::new(RungeKutta {}),
            };

            ensemble = physics::ensemble::Ensemble::new(
                set_atoms, //200_u64,
                set_boxlength,
                0_f64,
                set_step,
                set_temperature,
                lattice::LatticeType::FCC,
            );

        },

        cli::run_type::RunType::Resume {
            name: set_name,
            integrator: set_integrator
        } => {

            run_name = set_name;

            chosen_integrator = match set_integrator {
                integrators::SupportedIntegrator::Verlet => Box::new(Verlet {}),
                integrators::SupportedIntegrator::RungeKutta => Box::new(RungeKutta {}),
            };

            ensemble = MDIO::read_ensemble(&run_name).unwrap();
        },
    }
    
    return (ensemble,chosen_integrator,run_name);

}
