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

    let mut counter = 0_u64;
    while ensemble.t < 1e3 {

        if counter > 1000 {
            counter = 0;
            
            println!(
                "{} - {} - {} - {} - {}", 
                ensemble.t, 
                ensemble.atoms[0].position.x, 
                ensemble.atoms[1].position.x, 
                ensemble.atoms[0].velocity.x, 
                ensemble.atoms[1].velocity.x, 
            );

            // TODO: remove this after debugged the force
            let _ = io::write(&ensemble);
        }

        chosen_integrator.dynamics(
            dynamics, 
            &mut ensemble.atoms, 
            ensemble.t, 
            ensemble.dt,
            ensemble.box_length,
        );
        physics::ensemble::periodic_conditions(&mut ensemble);
        ensemble.t += ensemble.dt;
        counter += 1;

    }
}
