use crate::utils::UncertainFloat;

#[derive(Clone)]
pub struct Isotope {
    pub mass_number: u16,
    pub mass: UncertainFloat,
    pub abundance: UncertainFloat,
    pub b_coherent: UncertainFloat
}
