use crate::physics::atom::Atom;

pub fn verlet(
    differential_equation_system: fn(&mut Vec<Atom>,&Vec<Atom>,f64,f64),
    atoms: &mut Vec<Atom>,
    t: f64,
    dt: f64,
    box_length: f64,
) {

    // Definition and initialization of k
    let mut k1: Vec<Atom> = Atom::initialize(atoms.len()).clone();
    let mut k2: Vec<Atom> = Atom::initialize(atoms.len()).clone();

    differential_equation_system(&mut k1,&atoms,t,box_length);
    
    for i in 0..atoms.len() {
        atoms[i].position.x += atoms[i].velocity.x*dt + f64::powf(k1[i].velocity.x*dt, 2.)/2.;
    }
    
    differential_equation_system(&mut k2,&atoms,t+dt,box_length);
    
    for i in 0..atoms.len() {
        atoms[i].velocity.x += (k1[i].velocity.x + k2[i].velocity.x)*dt/2.
    }

}