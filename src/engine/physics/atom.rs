use crate::data_structure::trivector::Trivector;
use serde::{Serialize, Deserialize};

use std::ops::Add;
use std::ops::Mul;

#[derive(Serialize, Deserialize, Copy)]
pub struct Atom {
    pub position: Trivector,
    pub velocity: Trivector,
}

impl Clone for Atom {
    fn clone(&self) -> Self {
        Atom {
            position: self.position.clone(),
            velocity: self.velocity.clone(),
        }
    }
}

impl Add for Atom {
    type Output = Atom;
    fn add(self, rhs: Atom) -> Atom {
        Atom {
            position: self.position + rhs.position,
            velocity: self.velocity + rhs.velocity,
        }
    }
}
impl Mul<f64> for Atom {
    type Output = Atom;
    fn mul(self, rhs: f64) -> Atom {
        Atom {
            position: self.position * rhs,
            velocity: self.velocity * rhs,
        }
    }  
}

impl Atom {
    pub fn empty() -> Atom {
        Atom { position: Trivector::empty(), velocity: Trivector::empty() }
    }

    pub fn initialize(size: usize) -> Vec<Atom> {
        let mut atoms = Vec::<Atom>::with_capacity(size);
        for _ in 0..size {
            atoms.push(Atom::empty());
        }
        atoms
    }
}

pub fn sum_atoms_vec(result: &mut Vec<Atom>,atoms_1: &Vec<Atom>, atoms_2: &Vec<Atom>) {
    
    assert_eq!(atoms_1.len(),atoms_2.len());
    for i in 0..atoms_1.len() {
        result[i] = (*atoms_1)[i] + (*atoms_2)[i];
    }

}

pub fn times_scalar_atoms_vec(result: &mut Vec<Atom>, atoms: &Vec<Atom>, scalar: f64) {
    
    for i in 0..atoms.len() {
        result[i] = (*atoms)[i] * scalar;
    }
}