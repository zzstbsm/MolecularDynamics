use crate::engine::{data_structure::trivector::Trivector, physics::atom::Atom};

/// Move the senter of mass to a not moving one
pub fn normalize_velocity(atoms: &mut Vec<Atom>) {

    let mut offset: Trivector = Trivector::empty();

    let number_of_atoms: usize = atoms.len();

    for i in 0..number_of_atoms {
        offset += atoms[i].velocity;
    }

    // Normalize velocity over the number of atoms
    offset /= number_of_atoms as f64;

    for i in 0..number_of_atoms {
        atoms[i].velocity -= offset;
    }
}
