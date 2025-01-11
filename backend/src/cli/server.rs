use clap::Parser;

#[derive(Parser)]
pub struct ServerTypeArgs {

    #[arg(short,long)]
    /// Set the port where the port is listening
    pub port: u64
    
}

