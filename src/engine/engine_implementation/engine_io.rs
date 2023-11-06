use super::Engine;

impl Engine {

    pub fn save(
        &self,
        filename: &str,
    ) {
        
        let mut path: std::path::PathBuf = std::path::PathBuf::new();
        path.push(filename);
        let _ = self.ensemble.save(&path);
        println!("Ensemble saved in file!");
    }

}