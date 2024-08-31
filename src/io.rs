use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

use crate::physics::ensemble::Ensemble;

/// MolecularDynamics input-output handling
pub struct MDIO {}

impl MDIO {

    /// Get the File buffer in order to do IO operations
    /// new == true creates a new file, overwriting the old one if it exists
    /// new == false appends the content
    fn get_buffer(filename: &str, new: bool) -> File {

        return match new {
            true => {

                let path = std::path::Path::new(filename);
                let prefix = path.parent().unwrap();
                std::fs::create_dir_all(prefix).unwrap();

                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(filename)
                    .unwrap()
            }
            false => {
                OpenOptions::new()
                    .append(true)
                    .open(filename)
                    .unwrap()
            }
        }

    }

    /// Get the properties of the ensemble. If the preamble is true, return the preamble only
    pub fn get_properties(ensemble: &Ensemble, preamble: bool) -> String {

        return match preamble {
            true => {
                format!(
                    "{},{},{},{},{},{}\n",
                    "time",
                    "total_energy",
                    "kinetic_energy",
                    "potential_energy",
                    "pressure",
                    "real_temperature",
                )
            }
            false => {
                let properties = ensemble.get_properties();

                format!(
                    "{},{},{},{},{},{}\n",
                    ensemble.t,
                    properties.total_energy,
                    properties.kinetic_energy,
                    properties.potential_energy,
                    properties.pressure,
                    properties.real_temperature,
                )
            }
        }
    }

    /// Write in a file the properties of a file. If the file does not exist, create it and add a
    /// premble
    pub fn write_properties(run_name: &str, ensemble: &Ensemble, preamble: bool) -> String {

        let filename = FileNamingConvention::get_properties_name(run_name);
        
        let properties_string = Self::get_properties(ensemble, preamble);
        let mut file = Self::get_buffer(&filename, preamble);

        let _ = write!(
            file,
            "{}",
            properties_string,
        );

        // If the preamble was true, it wrote only the preamble
        // Write also the first line of properties
        if preamble {
            return Self::write_properties(&run_name, ensemble, !preamble);
        }

        return properties_string;

    }

    /// Get a lattice saved from a previous run
    pub fn read_ensemble(run_name: &str) -> Ensemble{

        let filename = FileNamingConvention::get_ensemble_name(run_name);

        let file = Self::get_buffer(&filename, false);
        let reader = BufReader::new(file);

        let ensemble: Ensemble = serde_json::from_reader(reader).unwrap();
        return ensemble;
    }

    /// Save the lattice of the run
    pub fn write_ensemble(run_name: &str, ensemble: &Ensemble) {

        let filename = FileNamingConvention::get_ensemble_name(run_name);

        let mut file = Self::get_buffer(&filename, true);

        let _ = write!(
            file,
            "{}",
            serde_json::to_string(&ensemble).unwrap()
        );
        return;
    }

}

struct FileNamingConvention {}

impl FileNamingConvention {
    pub fn get_properties_name(run_name: &str) -> String {
        return format!("result/{run_name}/properties.csv");
    }
    pub fn get_ensemble_name(run_name: &str) -> String {
        return format!("result/{run_name}/ensemble.json");
    }
}
