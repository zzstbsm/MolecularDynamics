pub mod static_parameters;

use self::static_parameters::_STATIC_PARAMETERS;

use super::atom::Atom;
use crate::data_structure::trivector::Trivector;

pub type DynamicsType =  fn(&mut Vec<Atom>,&Vec<Atom>,f64,&f64);

/// System of differential equations that describe the dynamics of the ensemble
pub fn dynamics(
    state_derivative: &mut Vec<Atom>, 
    initial_state: &Vec<Atom>, 
    _dt: f64,
    box_length: &f64,
) {
    let number_of_atoms = initial_state.len();

    let force = compute_force(initial_state,&box_length);

    for index_atom in 0..number_of_atoms {

        state_derivative[index_atom].position = initial_state[index_atom].velocity;
        state_derivative[index_atom].velocity = force[index_atom];
    }

}

/// Compute the force between the atoms of the ensemble
fn compute_force(state: &Vec::<Atom>,box_length: &f64) -> Vec::<Trivector> {

    let number_of_atoms = state.len();
    let mut force = vec![Trivector::empty(); number_of_atoms];

    for index_atom_1 in 0..(number_of_atoms-1) {
        for index_atom_2 in (index_atom_1+1)..number_of_atoms {
            
            let force_one_atom = lennard_jones_force(
                &state[index_atom_1].position, 
                &state[index_atom_2].position,
                box_length,
            );
            
            force[index_atom_1] += force_one_atom;
            force[index_atom_2] -= force_one_atom;
        }
    }
    return force;
}

/// Compute the force between two atoms using the Lennard Jones potential
pub fn lennard_jones_force(position_1: &Trivector, position_2: &Trivector, box_length:&f64) -> Trivector {

    if Trivector::if_skip_distance_computation(position_1, position_2, box_length,_STATIC_PARAMETERS.force_cutoff) {
        return Trivector { x: 0_f64, y: 0_f64, z: 0_f64 };
    }

    let direction = Trivector::vec_distance(position_1, position_2, box_length);
    
    let distance = Trivector::distance_from_vec_distance(&direction);

    let multiply_factor = if distance > _STATIC_PARAMETERS.force_cutoff {
        0_f64
    }
    else {
        let force_minimum_over_distance = _STATIC_PARAMETERS.sigma / distance;
        - 4_f64 * _STATIC_PARAMETERS.epsilon * (
               (
                   f64::powf(force_minimum_over_distance,12_f64) * 12_f64 
                   - f64::powf(force_minimum_over_distance,6_f64) * 6_f64
               ) / f64::powf(distance,2_f64)
           )
    };

    return direction * multiply_factor;
}

/// Compute the Lennard Jones potential
pub fn lennard_jones_potential(position_1: &Trivector, position_2: &Trivector, box_length: &f64) -> f64 {

    if Trivector::if_skip_distance_computation(position_1, position_2, box_length,_STATIC_PARAMETERS.force_cutoff) {
        return 0_f64;
    }

    let distance: f64 = Trivector::distance(position_1, position_2, box_length);

    return if distance > _STATIC_PARAMETERS.force_cutoff {
        0_f64
    }
    else {
        let force_minimum_over_distance = _STATIC_PARAMETERS.sigma / distance;
        4_f64 * _STATIC_PARAMETERS.epsilon * (
            (
                f64::powf(force_minimum_over_distance,12_f64) 
                - f64::powf(force_minimum_over_distance,6_f64)
                - (
                    f64::powf(_STATIC_PARAMETERS.sigma/_STATIC_PARAMETERS.force_cutoff,12_f64) 
                    - f64::powf(_STATIC_PARAMETERS.sigma/_STATIC_PARAMETERS.force_cutoff,6_f64)
                )
            )
        )
    }
}
