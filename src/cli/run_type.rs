use clap::Subcommand;

use super::super::engine::integrators::SupportedIntegrator;

#[derive(Subcommand)]
pub enum RunType {

    /// Starts a new run with id `name`
    New {
        name: Option<String>,

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
        /// Set the integrator to use.
        integrator: SupportedIntegrator
    },

    /// Resumes a previous run with id `name`
    Resume {
        /// Insert the name of the run to resume
        name: Option<String>
    },

    // Import {
    //     format: SupportedExportFormat,
    //     data: ,
    // }
}
