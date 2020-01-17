use crate::Isotope;

pub struct Element {
    pub atomic_number: u8,
    pub name: &'static str,
    pub symbol: &'static str,
    pub mass: f64,
    pub isotopes: Vec<Isotope>,
    pub common_ions: Vec<i8>,
    pub oxidation_states: Vec<i8>
}