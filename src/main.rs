mod cli;
mod engine;

use anyhow::Result;
use clap::Parser;
// use indicatif::ProgressBar;

use cli::Cli;
use engine::Engine;
use engine::engine_parameters::RunSimulationParameters;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _args = Cli::parse();
    let filename = "prova.csv";

    let engine_parameters: RunSimulationParameters = RunSimulationParameters::default();
    // TODO: Engine set parameters

    let mut engine = Engine::new(engine_parameters);

    engine.run(100000_u64);
    engine.print_on_screen();
    engine.save(filename);

    return Ok(());
}
