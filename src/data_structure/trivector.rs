use std::fmt::Debug;

use rand::distributions::{Distribution, Uniform};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Trivector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Clone for Trivector {
    fn clone(&self) -> Self {
        Trivector { x: self.x, y: self.y, z: self.z }
    }
}

impl Debug for Trivector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}; y: {}; z: {}", self.x, self.y, self.z)
    }
}

impl Trivector {
    pub fn empty() -> Trivector {
        Trivector {
            x: 0_f64,
            y: 0_f64,
            z: 0_f64,
        } 
    }

    pub fn vec_distance(vector_1: &Trivector, vector_2: &Trivector, box_length: f64) -> Trivector {
        let mut distance = Trivector {
            x: vector_2.x - vector_1.x,
            y: vector_2.y - vector_1.y,
            z: vector_2.z - vector_1.z,
        };

        if distance.x > box_length/2. {
            distance.x -= box_length;
        } else if distance.x < -box_length/2. {
            distance.x += box_length;
        }

        if distance.y > box_length/2. {
            distance.y -= box_length;
        } else if distance.y < -box_length/2. {
            distance.y += box_length;
        }

        if distance.z > box_length/2. {
            distance.z -= box_length;
        } else if distance.z < -box_length/2. {
            distance.z += box_length;
        }
        return distance;
    }

    pub fn dot_product(vector_1: &Trivector, vector_2: &Trivector) -> f64 {
        vector_1.x * vector_2.x + vector_1.y * vector_2.y + vector_1.z * vector_2.z
    }

    #[allow(dead_code)]
    pub fn distance(vector_1: &Trivector, vector_2: &Trivector, box_length: f64) -> f64 {
    
         let distance_vector = Trivector::vec_distance(vector_1, vector_2,box_length);
        
        Trivector::dot_product(&distance_vector,&distance_vector).sqrt()
    }

    pub fn distance_from_vec_distance(distance_vector: &Trivector) -> f64 {
        Trivector::dot_product(distance_vector,distance_vector).sqrt()
    }

    pub fn sum(vector_1: &Trivector, vector_2: &Trivector) -> Trivector {
        Trivector {
            x: vector_1.x + vector_2.x,
            y: vector_1.y + vector_2.y,
            z: vector_1.z + vector_2.z
        }
    }

    pub fn sub(vector_1: &Trivector, vector_2: &Trivector) -> Trivector {
        Trivector {
            x: vector_1.x - vector_2.x,
            y: vector_1.y - vector_2.y,
            z: vector_1.z - vector_2.z
        }
    }

    pub fn times_scalar(&self, scalar: f64) -> Trivector {
        Trivector {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,    
        }
    }

    pub fn divided_scalar(&self, scalar: f64) -> Trivector {
        Trivector {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }

}

pub fn random_initialization() -> Trivector {

    let mut rng = rand::thread_rng();

    Trivector {
        x: Uniform::from(0_f64..1_f64).sample(&mut rng),
        y: Uniform::from(0_f64..1_f64).sample(&mut rng),
        z: Uniform::from(0_f64..1_f64).sample(&mut rng)
    }
}