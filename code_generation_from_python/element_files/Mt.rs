use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 109,
            name: "Meitnerium",
            symbol: "Mt",
            mass: 268.0,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: None,

            isotopes:
                vec![
                    Isotope { 
                        mass_number: 265,
                        mass: UncertainFloat::new(265.136_57_f64, 0.000_50_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 266,
                        mass: UncertainFloat::new(266.137_94_f64, 0.000_38_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 267,
                        mass: UncertainFloat::new(267.137_53_f64, 0.000_62_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 268,
                        mass: UncertainFloat::new(268.138_82_f64, 0.000_34_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 269,
                        mass: UncertainFloat::new(269.139_11_f64, 0.000_59_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 270,
                        mass: UncertainFloat::new(270.140_72_f64, 0.000_66_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 271,
                        mass: UncertainFloat::new(271.141_23_f64, 0.000_65_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    