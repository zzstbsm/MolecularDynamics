mod fcc;
mod simple_cubic;

use super::atom::Atom;

#[allow(dead_code)]
pub enum LatticeType {
    FCC,
    SimpleCubic,
}

pub trait Lattice {

    fn create(
        self: &Self,
        number_of_atoms: &u64,
        box_length: &f64,
    ) -> Vec<Atom>;
}


pub fn lattice_check_position(
    atoms_per_row: &u64, 
    mut x: u64,
    mut y: u64,
    mut z: u64
) -> (u64,u64,u64) {
    if x >= *atoms_per_row && y < *atoms_per_row {
        x = 0_u64;
        y += 1_u64;
    } 
    if y >= *atoms_per_row  && z < *atoms_per_row {
        y = 0_u64;
        z += 1_u64;
    };
    return (x,y,z);
}

pub fn lattice_create(
    atoms: &mut Vec<Atom>,
    number_of_atoms: &u64,
    box_length: &f64,
    lattice_type: LatticeType,
) {
    
    let lattice: Box<dyn Lattice> = match lattice_type {
        LatticeType::FCC => Box::<fcc::FCC>::new(fcc::FCC::new()),
        LatticeType::SimpleCubic => Box::<simple_cubic::SimpleCubic>::new(simple_cubic::SimpleCubic::new())
    };

    *atoms = lattice.as_ref().create(number_of_atoms, box_length);
    
}
