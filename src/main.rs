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
    let chosen_integrator: &dyn Integrator;

    let _args = Cli::parse();
    
    match _args.run_type {
        cli::run_type::RunType::New { 
            name: _, 
            n_atoms: set_atoms, 
            integrator: set_integrator, 
            boxlength: set_boxlength,
            step: set_step,
            temperature: set_temperature
        } => {
            
            chosen_integrator = match set_integrator {
                integrators::SupportedIntegrator::Verlet => &Verlet {},
                integrators::SupportedIntegrator::RungeKutta => &RungeKutta {},
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
            return;
        },
    }

    let _ = io::write(&ensemble);
    println!("Ensemble saved in file!");

    while ensemble.t < 1e3 {
        
        ensemble.run_step(
            chosen_integrator,
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
