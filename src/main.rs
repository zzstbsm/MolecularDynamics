use crate::physics::integrator::Integrator;

mod data_structure;
mod physics;
mod io;

fn main() {

    let mut ensemble : physics::ensemble::Ensemble = physics::ensemble::Ensemble::new(
        256_u64, //200_u64,
        20_f64,
        0_f64,
        1e-3_f64,
        1_f64,
    );
    println!("Ensemble initialized!");

    println!("Setting integrator");

    let choose_integrator = Integrator::Verlet;
    let chosen_integrator = match choose_integrator {
        Integrator::Verlet => physics::integrator::verlet,
        Integrator::RungeKutta => physics::integrator::runge_kutta,
    };

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