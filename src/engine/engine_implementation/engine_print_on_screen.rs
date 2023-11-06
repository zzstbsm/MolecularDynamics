use super::Engine;

impl Engine {
    pub fn print_on_screen(&self) {
        println!(
            "{} - {} - {} - {} - {}", 
            self.ensemble.t, 
            self.ensemble.atoms[0].position.x, 
            self.ensemble.atoms[1].position.x, 
            self.ensemble.atoms[0].velocity.x, 
            self.ensemble.atoms[1].velocity.x, 
        );
    }
}