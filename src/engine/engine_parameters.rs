use super::physics::atom::Atom;
use super::integrators::select_integrator;
use super::integrators::DEFAULT_INTEGRATOR;
use super::integrators::SupportedIntegrator;

pub enum EngineRequiredParameters {
    BoxLenght,
    NumberOfAtoms,
    TargetTemperature,
    TimeStepDuration,
}

const ENGINE_DEFAULT_BOXLENGHT: f64 = 20_f64;
const ENGINE_DEFAULT_NUMBER_OF_ATOMS: u64 = 200_u64;
const ENGINE_DEFAULT_TARGET_TEMPERATURE: f64 = 20_f64;
const ENGINE_DEFAULT_TIME_STEP: f64 = 1e-3_f64;
const ENGINE_DEFAULT_INTEGRATOR: SupportedIntegrator = DEFAULT_INTEGRATOR;

const ENGINE_RUN_DURATION: f64 = 1_f64;

pub struct RunSimulationParameters{
    pub t_max: f64,
    pub chosen_integrator: fn(
        fn(&mut Vec<Atom>,&Vec<Atom>,f64,f64),
        &mut Vec<Atom>,
        f64,
        f64,
        f64,
    ),
    pub box_lenght: f64,
    pub number_of_atoms: u64,
    pub target_temperature: f64,
    pub time_step: f64,
}

impl RunSimulationParameters {
    pub fn default() -> Self {
        return RunSimulationParameters {
            t_max: ENGINE_RUN_DURATION,
            chosen_integrator: select_integrator(ENGINE_DEFAULT_INTEGRATOR),

            box_lenght: ENGINE_DEFAULT_BOXLENGHT,
            number_of_atoms: ENGINE_DEFAULT_NUMBER_OF_ATOMS,
            target_temperature: ENGINE_DEFAULT_TARGET_TEMPERATURE,
            time_step: ENGINE_DEFAULT_TIME_STEP,
        };
    }
}