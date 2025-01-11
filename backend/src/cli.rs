pub mod run_type;
pub mod server;
pub mod standalone;

use clap::Parser;
use run_type::RunType;
use server::ServerTypeArgs;
use standalone::{StandaloneType, StandaloneTypeArgs};

use crate::{engine::{integrators::{self, runge_kutta::RungeKutta, verlet::Verlet, Integrator}, physics::ensemble::Ensemble}, io::MDIO, run_instance::RunInstance, server::Server, standalone::Standalone};

#[derive(Parser)]
#[command(name = "MolecularDynamics by zzstbsm")]
#[command(version = "0.0.2")]
#[command(about)]
#[command(long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {

    #[command(subcommand)]
    pub run_type: RunType,

    #[arg(short,long)]
    pub verbose: bool,
}

pub fn get_server_parameters(server_args: ServerTypeArgs) -> Box<dyn RunInstance> {
    return Box::new(
       Server {
            port: server_args.port
       }
    )

}

pub fn get_standalone_parameters(standalone_args: StandaloneTypeArgs) -> Box<dyn RunInstance> {
    
    // Define run parameters
    let ensemble: Ensemble;
    let chosen_integrator: Box<dyn Integrator>;
    let run_name: String;

    match standalone_args.sub {
        StandaloneType::New { 
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

            ensemble = Ensemble::new(
                set_atoms, //200_u64,
                set_boxlength,
                0_f64,
                set_step,
                set_temperature,
                crate::engine::physics::lattice::LatticeType::FCC,
            );
        },
        StandaloneType::Resume {
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
    
    return Box::new(
        Standalone {
            ensemble,
            chosen_integrator,
            run_name,
        }
    )

}

pub fn get_instance_from_cli(_args: Cli) -> Box<dyn RunInstance> {

    match _args.run_type {
        RunType::Standalone(standalone_args) => {
            return get_standalone_parameters(standalone_args);
        }
        RunType::Server(server_args) => {
            return get_server_parameters(server_args)
        }
    }
}
