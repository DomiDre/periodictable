use crate::utils::UncertainFloat;

/// Trait for basic info of every element
pub trait ElementNeutronScattering {
    fn b_coherent(&self) -> UncertainFloat;
} 
