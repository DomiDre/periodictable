use crate::utils::UncertainFloat;

#[derive(Clone)]
pub struct Mass {
    pub mass_number: u16,
    pub mass: UncertainFloat,
    pub abundance: UncertainFloat,
}
