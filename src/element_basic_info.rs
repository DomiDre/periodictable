use crate::mass::Mass;

/// Trait for basic info of every element
pub trait ElementBasicInfo {
    fn atomic_number(&self) -> u8;
    fn name(&self) -> &'static str;
    fn symbol(&self) -> &'static str;
    fn isotopes(&self) -> Vec<Mass>;
    fn common_ions(&self) -> Vec<i8> {
        vec![]
    }
    fn oxidation_states(&self) -> Vec<i8> {
        vec![]
    }
} 
