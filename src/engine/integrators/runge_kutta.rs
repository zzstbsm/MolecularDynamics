use crate::physics::atom;
use crate::physics::atom::Atom;

pub fn runge_kutta(
    differential_equation_system: fn(&mut Vec<Atom>,&Vec<Atom>,f64,f64),
    atoms: &mut Vec<Atom>,
    t: f64,
    dt: f64,
    box_length: f64,
) {

    const NUMBER_OF_NODES: usize = 4;

    let nodes: [f64; NUMBER_OF_NODES] = [0.,0.5,0.5,1.];
    let weights: [f64; NUMBER_OF_NODES] = [1./6.,1./3.,1./3.,1./6.];

    let mut intermediate_state = Atom::initialize(atoms.len());

    let mut buffer_k = Atom::initialize(atoms.len());

    // Definition and initialization of k
    let mut k: Vec<Vec<Atom>> = Vec::<Vec::<Atom>>::with_capacity(NUMBER_OF_NODES + 1);
    for _ in 0..NUMBER_OF_NODES+1 {
        k.push(Atom::initialize(atoms.len()).clone())
    }

    for i in 0..NUMBER_OF_NODES {

        atom::times_scalar_atoms_vec(&mut buffer_k,&k[i],dt*nodes[i]);
        atom::sum_atoms_vec(&mut intermediate_state,&atoms, &buffer_k);
        
        let tt = t + dt*nodes[i];

        differential_equation_system(&mut k[i+1],&intermediate_state,tt,box_length);
    }
    
    // Weights part
    for i in 0..NUMBER_OF_NODES {
        atom::times_scalar_atoms_vec(&mut buffer_k, &k[i+1], weights[i]*dt);
        intermediate_state = atoms.clone();
        atom::sum_atoms_vec(atoms, &intermediate_state, &buffer_k)
    }

}