use serde::{Serialize, Deserialize};

use crate::data_structure::trivector::Trivector;
use crate::physics::atom::Atom;
use super::atom;
use super::lattice;
use super::lattice::LatticeType;

#[derive(Serialize, Deserialize)]
pub struct Ensemble {
    pub atoms: Vec::<atom::Atom>,
    pub box_length: f64,
    pub number_of_atoms: u64, 

    pub t: f64,
    pub dt: f64,

    pub target_temperature: f64,
}

impl Clone for Ensemble {
    fn clone(&self) -> Self {
        Ensemble {
            atoms: self.atoms.clone(),
            box_length: self.box_length,
            number_of_atoms: self.number_of_atoms,
            t: self.t,
            dt: self.dt,
            target_temperature: self.target_temperature,
        }
    }
}

impl Ensemble {
    
    pub fn new(
        number_of_atoms: u64,
        box_length: f64,
        t: f64,
        dt: f64,
        target_temperature: f64,
        lattice_type: LatticeType,
    ) -> Ensemble {

        let mut atoms: Vec<Atom> = Vec::<Atom>::with_capacity(number_of_atoms as usize);

        if number_of_atoms == 2_u64 {
            return initialization_two_atoms(
                2_u64,
                box_length,
                0_f64,
                1e-5_f64,
                1_f64,
            );
        }

        lattice::lattice_create(
            &mut atoms,
            &number_of_atoms,
            &box_length,
            lattice_type,
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

    pub fn to_csv(&self,preamble: bool) -> String {

        let mut to_csv_str = String::new();
        
        if preamble {
            to_csv_str.push_str(
                "t,atom,x,y,z,vx,vy,vz\n"
            );
        }

        for index in 0..self.atoms.len() {
            let atom = &self.atoms[index];
            to_csv_str.push_str(
                format!(
                    "{},{},{},{},{},{},{},{}\n",
                    self.t,
                    index,
                    atom.position.x,
                    atom.position.y,
                    atom.position.z,
                    atom.velocity.x,
                    atom.velocity.y,
                    atom.velocity.z,
                ).as_str()
            );
        }

        return to_csv_str;
    }

}

fn initialization_two_atoms(
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

pub fn periodic_conditions(ensemble: &mut Ensemble) {

    for index in 0..ensemble.atoms.len() {
        let position = &mut ensemble.atoms[index].position;

        if position.x > ensemble.box_length {
            position.x -= ensemble.box_length;
        } else if position.x < 0. {
            position.x += ensemble.box_length;
        }

        if position.y > ensemble.box_length {
            position.y -= ensemble.box_length;
        } else if position.y < 0. {
            position.y += ensemble.box_length;
        }

        if position.z > ensemble.box_length {
            position.z -= ensemble.box_length;
        } else if position.z < 0. {
            position.z += ensemble.box_length;
        }

    }

}