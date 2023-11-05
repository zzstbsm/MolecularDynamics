mod cli;
mod data_structure;
mod integrators;
mod physics;

use anyhow::{Context,Result};
use clap::Parser;
// use indicatif::ProgressBar;

use cli::Cli;
use physics::lattice as lattice;

#[allow(unreachable_code,unused)]
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Cli::parse();

    return Ok(());

    let run_simulation_parameters = RunSimulationParameters {
        t_max: 1_f64
    };
    run_simulation(
        &args.destination_path,
        run_simulation_parameters,
    );

}

fn run_simulation(
    save_directory: &std::path::PathBuf,
    run_simulation_parameters: RunSimulationParameters,
) {
    

    let mut ensemble : physics::ensemble::Ensemble = physics::ensemble::Ensemble::new(
        256_u64, //200_u64,
        20_f64,
        0_f64,
        1e-3_f64,
        1_f64,
    );
    println!("Ensemble initialized!");

    println!("Setting integrator");

    let choose_integrator = integrators::SupportedIntegrator::Verlet;
    let chosen_integrator = match choose_integrator {
        integrators::SupportedIntegrator::Verlet => integrators::verlet::verlet,
        integrators::SupportedIntegrator::RungeKutta => integrators::runge_kutta::runge_kutta,
    };

    let _ = ensemble.save(save_directory);
    println!("Ensemble saved in file!");

    let mut counter = 0_u64;
    while ensemble.t < run_simulation_parameters.t_max {

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
            let _ = ensemble.save(save_directory);
        }

        chosen_integrator(
            physics::dynamics, 
            &mut ensemble.atoms, 
            ensemble.t, 
            ensemble.dt,
            ensemble.box_length,
        );
        physics::ensemble::periodic_conditions(&mut ensemble);
        ensemble.t += ensemble.dt;
        counter += 1;

    }
    println!("Successful!");
}

struct RunSimulationParameters{
    t_max: f64,
}