use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 113,
        name: "Nihonium",
        symbol: "Nh",
        mass: 286.0,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: None,

        isotopes: vec![
            Isotope { 
                mass_number: 286,
                mass: UncertainFloat::new(0.0, 0.0),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
