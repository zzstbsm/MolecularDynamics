use clap::Parser;
use engine::integrators::{verlet::Verlet,runge_kutta::RungeKutta};

mod cli;
use crate::cli::Cli;

mod engine;
use crate::engine::data_structure;
use crate::engine::integrators;
use crate::engine::io;
use crate::engine::integrators::Integrator;
use crate::engine::physics;
use crate::engine::physics::dynamics::dynamics;
use crate::engine::physics::ensemble::Ensemble;
use crate::engine::physics::lattice;


fn main() {

    // Define run parameters
    let mut ensemble: Ensemble;
    let chosen_integrator: Box<dyn Integrator>;

    let _args = Cli::parse();

    (ensemble, chosen_integrator) = apply_cli_parameters(_args);
    
    
    let _ = io::write(&ensemble);
    println!("Ensemble saved in file!");

    while ensemble.t < 1e3 {
        
        ensemble.run_step(
            &(*chosen_integrator),
            dynamics,
            1000
        );
        let properties = ensemble.get_properties();
        println!(
            "{} | {} | {} | {} | {} | {}",
            ensemble.t,
            properties.total_energy,
            properties.kinetic_energy,
            properties.potential_energy,
            properties.pressure,
            properties.real_temperature,
        );

    }
}

/// Prova
fn apply_cli_parameters(_args: Cli) -> (Ensemble, Box<dyn Integrator>) {

    // Define run parameters
    let ensemble: Ensemble;
    let chosen_integrator: Box<dyn Integrator>;


    match _args.run_type {
        cli::run_type::RunType::New { 
            name: _, // TODO implement run_name when saving
            n_atoms: set_atoms, 
            integrator: set_integrator, 
            boxlength: set_boxlength,
            step: set_step,
            temperature: set_temperature
        } => {
            
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

        cli::run_type::RunType::Resume { name: _ } => {
            // TODO implementation

            chosen_integrator = Box::new(Verlet {});

            ensemble = physics::ensemble::Ensemble::new(
                2_u64, //200_u64,
                20_f64,
                0_f64,
                0_f64,
                1_f64,
                lattice::LatticeType::FCC,
            );
        },
    }
    
    return (ensemble,chosen_integrator);

}
