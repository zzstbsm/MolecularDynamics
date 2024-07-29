use super::Ensemble;

/// Compute the periodic conditions of the ensemble
/// The atoms outside the box after a step are being reput into the box
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
