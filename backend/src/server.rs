mod rest_api;
mod server_start;
mod server_error;

use crate::run_instance::RunInstance;

pub struct Server {
    pub port: u16
}

impl RunInstance for Server {
    fn start(&mut self) {
        println!("Server started on port {}", self.port);
        server_start::server_main(self.port);
        return;
    }
}
