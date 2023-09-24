use std::fs::{create_dir,OpenOptions, File};
use std::io::Write;

use crate::physics::ensemble::Ensemble;

pub fn write(ensemble: &Ensemble) -> std::io::Result<()>{

    let _ = create_dir("result");
    let filename = "result/two.csv";


        
    let (mut file, exists) = match OpenOptions::new().read(true).open(&filename) {
        Ok(_) => {
            let file = OpenOptions::new().append(true).open(&filename).unwrap();
            (file, true)
        },
        Err(_) => {
            let file = File::create(&filename).unwrap();
            (file,false)
        },
    };

    write!(
        file,
        "{}",
        ensemble.to_csv(!exists)
    )
}