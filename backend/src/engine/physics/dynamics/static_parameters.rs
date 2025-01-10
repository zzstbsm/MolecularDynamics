
pub struct StaticParameters {
    pub sigma: f64,
    pub epsilon: f64,
    pub mass: f64,
    pub force_cutoff: f64,
}

pub static _STATIC_PARAMETERS: StaticParameters = StaticParameters {
    sigma: 1_f64,
    epsilon: 1_f64,
    mass: 1_f64,
    force_cutoff: 2_f64,
};
