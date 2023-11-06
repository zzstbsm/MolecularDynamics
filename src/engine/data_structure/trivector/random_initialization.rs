use super::Trivector;

use rand::distributions::{Distribution, Uniform};

impl Trivector {

    pub fn random_initialization() -> Trivector {
    
        let mut rng = rand::thread_rng();
    
        Trivector {
            x: Uniform::from(0_f64..1_f64).sample(&mut rng),
            y: Uniform::from(0_f64..1_f64).sample(&mut rng),
            z: Uniform::from(0_f64..1_f64).sample(&mut rng)
        }
    }
    
}