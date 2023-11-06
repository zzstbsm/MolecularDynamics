use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub destination_path: std::path::PathBuf,
}

#[derive(Debug)]
pub struct CliError {

}