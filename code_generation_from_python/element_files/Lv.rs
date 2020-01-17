use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 116,
            name: "Livermorium",
            symbol: "Lv",
            mass: 293.0,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: None,

            isotopes:
                vec![
                    Isotope { 
                        mass_number: 293,
                        mass: UncertainFloat::new(0.0, 0.0),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    