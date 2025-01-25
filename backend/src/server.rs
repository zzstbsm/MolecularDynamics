mod openapi;
mod rest_api;
mod server_config;
mod server_error;
mod server_start;

use crate::run_instance::RunInstance;

pub struct Server {
    pub port: u16
}

impl RunInstance for Server {
    fn start(&mut self) {
        println!("Starting backend server");
        server_start::server_main(self.port);
        return;
    }
}
