use crate::data_structure::trivector;
use crate::data_structure::trivector::Trivector;

use super::Lattice;

use super::super::atom::Atom;
use super::lattice_check_position;

pub struct FCC {}

impl FCC {
    pub fn new() -> Self {
        FCC{}
    }
}

impl Lattice for FCC {

    fn create(
        self: &Self,
        number_of_atoms: &u64,
        box_length: &f64,
    ) -> Vec<Atom> {

        let mut atoms = Vec::<Atom>::with_capacity(*number_of_atoms as usize);

        const ATOMS_PER_CELL: u64 = 4;
        let atoms_per_row = (*number_of_atoms as f64 / ATOMS_PER_CELL as f64).cbrt().ceil() as u64;
    
        let mut inserted_atoms: u64 = 0;
        let (mut x, mut y, mut z) = (0_u64, 0_u64, 0_u64);
        
        let mut inserted_in_grid: u64 = 1;
    
        while inserted_atoms < *number_of_atoms {
    
            if inserted_in_grid > ATOMS_PER_CELL { 
                inserted_in_grid = 1;
                x += 1;
            }
            
            let offset: Trivector = fcc_calc_offset(inserted_in_grid);
            (x,y,z) = lattice_check_position(&atoms_per_row, x, y, z);
            
    
            let position: Trivector = Trivector {
                x: x as f64 + offset.x,
                y: y as f64 + offset.y,
                z: z as f64 + offset.z,
            } * (*box_length) / (atoms_per_row as f64);
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
    
        return atoms;
    }


}

fn fcc_calc_offset(inserted_in_grid: u64) -> Trivector{
    return match inserted_in_grid {
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
}