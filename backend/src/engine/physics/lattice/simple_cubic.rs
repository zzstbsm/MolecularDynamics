use crate::data_structure::trivector;
use crate::data_structure::trivector::Trivector;

use super::super::atom::Atom;
use super::{lattice_check_position, Lattice};

pub struct SimpleCubic {}

impl SimpleCubic {
    pub fn new() -> Self {
        SimpleCubic {}
    }
}

impl Lattice for SimpleCubic {
    fn create(
        &self,
        number_of_atoms: &u64,
        box_length: &f64
    ) -> Vec<Atom>{

        let mut atoms = Vec::<Atom>::with_capacity(*number_of_atoms as usize);

        let mut inserted_atoms: u64 = 0;
        let atoms_per_row = (*number_of_atoms as f64).cbrt().ceil() as u64;
        let (mut x, mut y, mut z) = (0_u64, 0_u64, 0_u64);
    
        while inserted_atoms < *number_of_atoms {
            
            (x,y,z) = lattice_check_position(&atoms_per_row, x, y, z);
            
            let position: Trivector = Trivector {
                x: x as f64,
                y: y as f64,
                z: z as f64,
            } * (*box_length) / (atoms_per_row as f64);
    
            let velocity = trivector::random_initialization();
    
            atoms.push(
                Atom {
                    position,
                    velocity,
                }
            );
    
            inserted_atoms += 1;
            x += 1;
        }
    
        return atoms;
    }
}