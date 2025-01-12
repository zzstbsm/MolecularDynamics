use clap::{Parser, Subcommand};

use crate::engine::api::SupportedIntegrator;

#[derive(Parser)]
pub struct StandaloneTypeArgs {

    #[command(subcommand)]
    pub sub: StandaloneType,
}

#[derive(Subcommand)]
pub enum StandaloneType {
    /// Starts a new run with id `name`
    New {
        #[arg(long="run-name")]
        /// Set name of the run
        name: String,

        #[arg(long="set-atoms")]
        /// Set number of atoms in the ensemble
        n_atoms: u64,
        
        #[arg(long="set-boxlength")]
        /// Set the size of the ensemble
        boxlength: f64,
        
        #[arg(long="set-step")]
        /// Set the temporal step of the ensemble
        step: f64,

        #[arg(long="set-temperature")]
        /// Set the temperature of the ensemble
        temperature: f64,
        
        #[arg(long = "set-integrator")]
        /// Set the integrator to use
        integrator: SupportedIntegrator
    },

    /// Resumes a previous run with id `name`
    Resume {

        #[arg(long="run-name")]
        /// Insert the name of the run to resume
        name: String,
        
        #[arg(long="set-integrator")]
        /// Set the integrator to use
        integrator: SupportedIntegrator
    },   
}
