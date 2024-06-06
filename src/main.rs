mod engine;

fn main() {
    return 0;
} 

// use engine::data_structure;
// use engine::integrators;
// use engine::io;
// use engine::physics;
//
// use engine::physics::lattice as lattice;

// fn main() {
//
//     let mut ensemble : physics::ensemble::Ensemble = physics::ensemble::Ensemble::new(
//         256_u64, //200_u64,
//         20_f64,
//         0_f64,
//         1e-3_f64,
//         1_f64,
//         lattice::LatticeType::FCC,
//     );
//     println!("Ensemble initialized!");
//
//     println!("Setting integrator");
//
//     let choose_integrator = integrators::SupportedIntegrator::Verlet;
//     let chosen_integrator = match choose_integrator {
//         integrators::SupportedIntegrator::Verlet => integrators::verlet::verlet,
//         integrators::SupportedIntegrator::RungeKutta => integrators::runge_kutta::runge_kutta,
//     };
//
//     let _ = io::write(&ensemble);
//     println!("Ensemble saved in file!");
//
//     let mut counter = 0_u64;
//     while ensemble.t < 1e3 {
//
//         if counter > 1000 {
//             counter = 0;
//             
//             println!(
//                 "{} - {} - {} - {} - {}", 
//                 ensemble.t, 
//                 ensemble.atoms[0].position.x, 
//                 ensemble.atoms[1].position.x, 
//                 ensemble.atoms[0].velocity.x, 
//                 ensemble.atoms[1].velocity.x, 
//             );
//
//             // TODO: remove this after debugged the force
//             let _ = io::write(&ensemble);
//         }
//
//         chosen_integrator(
//             physics::dynamics, 
//             &mut ensemble.atoms, 
//             ensemble.t, 
//             ensemble.dt,
//             ensemble.box_length,
//         );
//         physics::ensemble::periodic_conditions(&mut ensemble);
//         ensemble.t += ensemble.dt;
//         counter += 1;
//
//     }
//     println!("Successful!");
// }
