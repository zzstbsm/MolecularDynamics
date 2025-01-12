use clap::Parser;

mod cli;
use cli::get_instance_from_cli;
use crate::cli::Cli;

mod engine;
mod run_instance;

mod server;
mod standalone;

fn main() {

    let _args = Cli::parse();

    let mut instance = get_instance_from_cli(_args);

    instance.start();

    return;
}

