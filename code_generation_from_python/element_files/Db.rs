use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 105,
            name: "Dubnium",
            symbol: "Db",
            mass: 262.0,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: None,

            isotopes:
                vec![
                    Isotope { 
                        mass_number: 255,
                        mass: UncertainFloat::new(255.107_40_f64, 0.000_45_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 256,
                        mass: UncertainFloat::new(256.108_11_f64, 0.000_39_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 257,
                        mass: UncertainFloat::new(257.107_86_f64, 0.000_25_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 258,
                        mass: UncertainFloat::new(258.109_44_f64, 0.000_37_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 259,
                        mass: UncertainFloat::new(259.109_72_f64, 0.000_31_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 260,
                        mass: UncertainFloat::new(260.111_43_f64, 0.000_25_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 261,
                        mass: UncertainFloat::new(261.112_11_f64, 0.000_25_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 262,
                        mass: UncertainFloat::new(262.114_15_f64, 0.000_20_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 263,
                        mass: UncertainFloat::new(263.115_08_f64, 0.000_18_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 264,
                        mass: UncertainFloat::new(264.117_47_f64, 0.000_25_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 265,
                        mass: UncertainFloat::new(265.118_66_f64, 0.000_30_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    