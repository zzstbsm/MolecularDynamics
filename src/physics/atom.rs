use crate::data_structure::trivector::Trivector;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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

    pub fn sum(atom_1: &Atom, atom_2: &Atom) -> Atom {
        Atom {
            position: Trivector::sum(&atom_1.position,&atom_2.position),
            velocity: Trivector::sum(&atom_1.velocity,&atom_2.velocity),
        }
    }
}

pub fn sum_atoms_vec(result: &mut Vec<Atom>,atoms_1: &Vec<Atom>, atoms_2: &Vec<Atom>) {
    
    for i in 0..atoms_1.len() {
        result[i] =
            Atom::sum(&atoms_1[i], &atoms_2[i]);
    }

}

pub fn times_scalar_atoms_vec(result: &mut Vec<Atom>, atoms: &Vec<Atom>, scalar: f64) {
    
    for i in 0..atoms.len() {
        result[i].position = atoms[i].position.times_scalar(scalar);
        result[i].velocity = atoms[i].velocity.times_scalar(scalar);
    }
}