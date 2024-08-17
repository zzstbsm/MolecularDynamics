#!/bin/sh

INTEGRATOR="verlet"

ATOMS_NUMBER="200"
BOXLENGTH="20"
STEP="1e-3"
TEMPERATURE="1"

cargo run -r -- \
    new \
    --set-integrator $INTEGRATOR \
    --set-atoms $ATOMS_NUMBER \
    --set-boxlength $BOXLENGTH \
    --set-step $STEP \
    --set-temperature $TEMPERATURE
 
