use crate::engine::{data_structure::trivector::Trivector, physics::{atom::Atom, dynamics::static_parameters::_STATIC_PARAMETERS}};

/// Given a set of atoms, rescale the real temperature to hit the target_temperature
pub fn set_temperature(
    atoms: &mut Vec<Atom>,
    target_temperature: f64,
) {

    let mut current_temperature: f64 = 0_f64;

    let number_atoms = atoms.len();

    for i in 0..number_atoms as usize {
        current_temperature += Trivector::dot_product(&atoms[i].velocity, &atoms[i].velocity);
    }
    current_temperature *= _STATIC_PARAMETERS.mass / (3_f64 * number_atoms as f64);

    let temperature_rescaling = target_temperature / current_temperature;

    for i in 0..number_atoms as usize {
        atoms[i].velocity *= temperature_rescaling.sqrt();
    }

}
