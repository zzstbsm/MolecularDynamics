use crate::data_structure::trivector::Trivector;

use super::Ensemble;
use super::super::atom::Atom;

pub fn initialization_two_atoms(
    number_of_atoms: u64,
    box_length: f64,
    t: f64,
    dt: f64,
    target_temperature: f64
) -> Ensemble {

    let mut atoms: Vec<Atom> = Vec::<Atom>::with_capacity(number_of_atoms as usize);

    let velocity_1 = Trivector {
        x: 0.5,
        y: 0.,
        z: 0.,
    };
    let position_1 = Trivector {
        x: 0.25,
        y: 0.,
        z: 0.,
    } * box_length;

    let velocity_2 = Trivector {
        x: -0.5,
        y: 0.,
        z: 0.,
    };
    let position_2 = Trivector {
        x: 0.75,
        y: 0.,
        z: 0.,
    } * box_length;

    atoms.push(
        Atom { 
            position: position_1,
            velocity: velocity_1,
        }
    );
    atoms.push(
        Atom { 
            position: position_2,
            velocity: velocity_2,
        }
    );

    Ensemble {
        atoms,
        box_length,
        number_of_atoms,
        t,
        dt,
        target_temperature
    }
}
