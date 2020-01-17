use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 111,
            name: "Roentgenium",
            symbol: "Rg",
            mass: 272.0,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: None,

            isotopes:
                vec![
                    Isotope { 
                        mass_number: 272,
                        mass: UncertainFloat::new(272.153_48_f64, 0.000_36_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    