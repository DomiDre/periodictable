use crate::utils::UncertainFloat;

pub struct ScatteringFactor {
    // pub masses: [Vec<Mass>; 119]
}

pub struct ScatteringFactors {
    pub scattering_factors: [Vec<ScatteringFactor>; 97]
}

impl ScatteringFactors {
    pub fn new() -> ScatteringFactors {
        ScatteringFactor {
            
        }
    }
}
