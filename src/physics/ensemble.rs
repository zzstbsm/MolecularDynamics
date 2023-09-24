use serde::{Serialize, Deserialize};

use crate::data_structure::trivector;
use crate::data_structure::trivector::Trivector;
use crate::physics::atom::Atom;
use super::atom;

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
        target_temperature: f64
    ) -> Ensemble {

        const ATOMS_PER_CELL: u64 = 4;

        let mut inserted_atoms: u64 = 0;            
        let atoms_per_row = (number_of_atoms as f64 / ATOMS_PER_CELL as f64).cbrt().ceil() as u64;
        let mut atoms: Vec<Atom> = Vec::<Atom>::with_capacity(number_of_atoms as usize);

        let (mut x, mut y, mut z) = (0_u64, 0_u64, 0_u64);

        if number_of_atoms == 2_u64 {
            return initialization_two_atoms(
                2_u64,
                box_length,
                0_f64,
                1e-5_f64,
                1_f64,
            );
        }

        let mut inserted_in_grid = 1;

        while inserted_atoms < number_of_atoms {

            if inserted_in_grid > ATOMS_PER_CELL { 
                inserted_in_grid = 1;
                x += 1;
            }
            
            let offset: Trivector = match inserted_in_grid {
                1 => Trivector {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                2 => Trivector {
                    x: 0.5,
                    y: 0.5,
                    z: 0.,
                },
                3 => Trivector {
                    x: 0.,
                    y: 0.5,
                    z: 0.5,
                },
                4 => Trivector {
                    x: 0.5,
                    y: 0.,
                    z: 0.5,
                },
                _ => Trivector { x: 0., y: 0., z: 0. }
     
            };
            
            if x >= atoms_per_row && y < atoms_per_row {
                x = 0_u64;
                y += 1_u64;
            } 
            if y >= atoms_per_row  && z < atoms_per_row {
                y = 0_u64;
                z += 1_u64;
            };

            let position: Trivector = Trivector {
                x: x as f64 + offset.x,
                y: y as f64 + offset.y,
                z: z as f64 + offset.z,
            }.times_scalar(box_length)
            .divided_scalar(atoms_per_row as f64);
            let velocity = trivector::random_initialization();

            atoms.push(
                Atom {
                    position,
                    velocity,
                }
            );

            inserted_atoms += 1;
            inserted_in_grid += 1;
        }

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
    }.times_scalar(box_length);

    let velocity_2 = Trivector {
        x: -0.5,
        y: 0.,
        z: 0.,
    };
    let position_2 = Trivector {
        x: 0.75,
        y: 0.,
        z: 0.,
    }.times_scalar(box_length);

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