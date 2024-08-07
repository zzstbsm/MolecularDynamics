mod initialization_two_particles;
mod macroscopic_properties;
mod normalize_velocity;
mod periodic_conditions;
mod set_temperature;

use serde::{Serialize, Deserialize};

use crate::engine::data_structure::trivector::Trivector;
use crate::engine::integrators::Integrator;

use self::macroscopic_properties::Properties;
use self::initialization_two_particles::initialization_two_atoms;
use self::normalize_velocity::normalize_velocity;
use self::periodic_conditions::periodic_conditions;
use self::set_temperature::set_temperature;
use super::dynamics::static_parameters::_STATIC_PARAMETERS;
use super::dynamics::{self, lennard_jones_force, lennard_jones_potential};
use super::atom::Atom;
use super::lattice;
use super::lattice::LatticeType;

#[derive(Serialize, Deserialize)]
pub struct Ensemble {
    pub atoms: Vec::<Atom>,
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
    
    /// Create new ensemble
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
        
        normalize_velocity(&mut atoms);
        set_temperature(&mut atoms,target_temperature);

        Ensemble {
            atoms,
            box_length,
            number_of_atoms,
            t,
            dt,
            target_temperature
        }
    }

    /// Write the ensemble into a csv
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

    /// Run the simulation for the number of steps indicated in steps_to_do.
    /// Use the integrator chosen_integrator with the dynamics indicated in dynamics (that would be
    /// a system of differential equations)
    pub fn run_step(
        &mut self,
        chosen_integrator: &dyn Integrator,
        dynamics: dynamics::DynamicsType,
        steps_to_do: u64
    ) {
        for _i in 0..steps_to_do {
            chosen_integrator.dynamics(
                dynamics, 
                &mut self.atoms, 
                self.t, 
                self.dt,
                &self.box_length,
            );
            periodic_conditions(self);
            normalize_velocity(&mut self.atoms);
            self.t += self.dt;
        }
    }

    /// Computes the properties of pressure, temperature, total energy, kinetic energy and
    /// potential energy of the ensemble
    pub fn get_properties(&self) -> Properties {
        
        let mut pressure: f64;
        let real_temperature: f64;
        let total_energy: f64;
        let kinetic_energy: f64;
        let mut potential_energy: f64;

        // Compute kinetic energy
        let mut velocity_squared = 0_f64;
        for i in 0..self.number_of_atoms as usize {
            let velocity: Trivector = self.atoms[i].velocity;
            velocity_squared += Trivector::dot_product(&velocity,&velocity);
        }
        kinetic_energy = velocity_squared * _STATIC_PARAMETERS.mass / 2_f64;

        // Compute real_temperature
        real_temperature = velocity_squared / (3_f64 * self.number_of_atoms as f64);

        // Compute potential energy
        potential_energy = 0_f64;
        for index_atom_1 in 0..(self.number_of_atoms-1) as usize {
            for index_atom_2 in (index_atom_1+1)..self.number_of_atoms as usize {
                
                potential_energy += lennard_jones_potential(
                    &self.atoms[index_atom_1].position, 
                    &self.atoms[index_atom_2].position,
                    &self.box_length,
                );
            }
        }

        // Compute pressure
        pressure = 0_f64;
        for index_atom_1 in 0..(self.number_of_atoms-1) as usize {
            for index_atom_2 in (index_atom_1+1)..self.number_of_atoms as usize {
                
                let direction: Trivector = Trivector::vec_distance(
                    &self.atoms[index_atom_1].position, 
                    &self.atoms[index_atom_2].position,
                    &self.box_length,
                );

                let force_one_atom = lennard_jones_force(
                    &self.atoms[index_atom_1].position, 
                    &self.atoms[index_atom_2].position,
                    &self.box_length,
                );
                pressure += Trivector::dot_product(&force_one_atom, &direction)
            }
        }
        pressure = (pressure + 2_f64 * real_temperature) / 3_f64;

        // Compute total energy
        total_energy = kinetic_energy + potential_energy;

        return Properties { 
            pressure,
            real_temperature,
            total_energy,
            kinetic_energy,
            potential_energy,
        }
    }

}

