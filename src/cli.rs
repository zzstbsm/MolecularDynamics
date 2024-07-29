pub mod run_type;

use clap::Parser;
use run_type::RunType;

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

