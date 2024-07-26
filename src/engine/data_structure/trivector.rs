use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::ops::Neg;


use std::fmt::Debug;

use rand::distributions::{Distribution, Uniform};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy)]
pub struct Trivector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Trivector {

    type Output = Trivector;

    fn add(self, rhs: Trivector) -> Trivector {
        Trivector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
impl Add<f64> for Trivector {
    type Output = Trivector;
    fn add(self, rhs: f64) -> Trivector {
        Trivector {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
impl AddAssign for Trivector {
    fn add_assign(&mut self, rhs: Trivector) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }    
}
impl AddAssign<f64> for Trivector {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }    
}
impl Sub for Trivector {
    type Output = Trivector;
    fn sub(self, rhs: Trivector) -> Trivector {
        Trivector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Sub<f64> for Trivector {
    type Output = Trivector;
    fn sub(self, rhs: f64) -> Trivector {
        Trivector {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}
impl SubAssign for Trivector {
    fn sub_assign(&mut self, rhs: Trivector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }    
}
impl SubAssign<f64> for Trivector {
    fn sub_assign(&mut self, rhs: f64) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }    
}
impl Mul for Trivector {
    type Output = Trivector;
    fn mul(self, rhs: Trivector) -> Trivector {
        Trivector {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl Mul<f64> for Trivector {
    type Output = Trivector;
    fn mul(self, rhs: f64) -> Trivector {
        Trivector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Div<f64> for Trivector {
    type Output = Trivector;
    fn div(self, rhs: f64) -> Trivector {
        Trivector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Neg for Trivector {
    type Output = Trivector;
    fn neg(self) -> Trivector {
        Trivector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
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
        let mut distance = *vector_2 - *vector_1;

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

    pub fn distance(vector_1: &Trivector, vector_2: &Trivector, box_length: f64) -> f64 {
    
        let distance_vector = Trivector::vec_distance(vector_1, vector_2,box_length);
        
        Trivector::dot_product(&distance_vector,&distance_vector).sqrt()
    }

    pub fn distance_from_vec_distance(distance_vector: &Trivector) -> f64 {
        Trivector::dot_product(distance_vector,distance_vector).sqrt()
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
