pub mod operations;
pub mod random_initialization;
pub mod trivector_implementation;

use std::fmt::Debug;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy)]
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