use crate::utils::UncertainFloat;

#[derive(Clone)]
pub struct Mass {
    pub mass_number: u16,
    pub mass: UncertainFloat,
    pub abundance: UncertainFloat,
}

impl Mass {
    pub fn new(mass_number: u16, mass: UncertainFloat, abundance: UncertainFloat) -> Self {
        Self {
            mass_number,
            mass,
            abundance
        }
    }
}