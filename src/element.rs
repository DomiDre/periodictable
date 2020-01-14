use crate::mass::Mass;
use crate::utils::UncertainFloat;

#[derive(Clone)]
pub struct Element {
    pub atomic_number: u8,
    pub name: &'static str,
    pub symbol: &'static str,
    pub common_ions: Vec<i8>,
    pub oxidation_states: Vec<i8>,
    pub isotopes: Vec<Mass>
}

impl Element {
    pub fn new(
        atomic_number: u8,
        name: &'static str,
        symbol: &'static str,
        common_ions: Vec<i8>,
        oxidation_states: Vec<i8>,
    ) -> Element {
        
        Element {
            atomic_number,
            name,
            symbol,
            common_ions,
            oxidation_states,
            isotopes: Vec::new()
        }
    }

    /// Get the mass of the element averaged over the existing isotopes, weighted by their
    /// abundance
    pub fn mass(&self) -> UncertainFloat {
        let mut average_mass = 0.0;
        let mut average_uncertainty = 0.0;
        let mut summed_weights = 0.0;
        for isotope in self.isotopes.iter() {
            average_mass += isotope.abundance.value*isotope.mass.value;
            average_uncertainty += (isotope.abundance.value*isotope.mass.value).powi(2);
            summed_weights += isotope.abundance.value;
        }
        average_mass = average_mass / summed_weights;
        average_uncertainty = average_uncertainty.sqrt() / summed_weights;
        UncertainFloat::new(average_mass, average_uncertainty)
    }

    /// Get the isotope mass. If the mass number is not present in the data for the given
    /// element, None is returned
    pub fn isotope_mass(&self, mass_number: u16) -> Option<&Mass> {
        if let Some(idx) = self.isotopes.iter().position(|x| x.mass_number == mass_number) {
            Some(&self.isotopes[idx])
        } else {
            None
        }
    }
}

#[test]
fn create_element() {
    let el = Element::new(3, "Lithium", "Li", vec![1], vec![]);
    assert_eq!(el.name, "Lithium");
}
