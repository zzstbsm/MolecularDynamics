use clap::Subcommand;

use super::{server::ServerTypeArgs, standalone::StandaloneTypeArgs};

#[derive(Subcommand)]
pub enum RunType {

    /// Start an HTTP server as backend for the simulations
    #[command(subcommand_help_heading = "Server heading")]
    Server(ServerTypeArgs),

    /// Start the simulations directly, without spinning up the server
    #[command(subcommand_help_heading = "Standalone heading")]
    Standalone(StandaloneTypeArgs),

}

