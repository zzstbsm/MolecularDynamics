use crate::run_instance::RunInstance;

pub struct Server {
    pub port: u64
}

impl RunInstance for Server {
    /// TODO start server
    fn start(&mut self) {
        println!("Server started on port {}", self.port);
        return;
    }
}
