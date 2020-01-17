use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 100,
            name: "Fermium",
            symbol: "Fm",
            mass: 257.0,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: None,

            isotopes:
                vec![
                    Isotope { 
                        mass_number: 242,
                        mass: UncertainFloat::new(242.073_43_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 243,
                        mass: UncertainFloat::new(243.074_51_f64, 0.000_25_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 244,
                        mass: UncertainFloat::new(244.074_08_f64, 0.000_31_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 245,
                        mass: UncertainFloat::new(245.075_38_f64, 0.000_30_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 246,
                        mass: UncertainFloat::new(246.075_280_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 247,
                        mass: UncertainFloat::new(247.076_82_f64, 0.000_16_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 248,
                        mass: UncertainFloat::new(248.077_184_f64, 0.000_013_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 249,
                        mass: UncertainFloat::new(249.079_02_f64, 0.000_15_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 250,
                        mass: UncertainFloat::new(250.079_515_f64, 0.000_013_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 251,
                        mass: UncertainFloat::new(251.081_566_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 252,
                        mass: UncertainFloat::new(252.082_460_f64, 0.000_006_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 253,
                        mass: UncertainFloat::new(253.085_176_f64, 0.000_005_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 254,
                        mass: UncertainFloat::new(254.086_848_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 255,
                        mass: UncertainFloat::new(255.089_955_f64, 0.000_005_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 256,
                        mass: UncertainFloat::new(256.091_767_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 257,
                        mass: UncertainFloat::new(257.095_099_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 258,
                        mass: UncertainFloat::new(258.097_07_f64, 0.000_22_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 259,
                        mass: UncertainFloat::new(259.100_59_f64, 0.000_30_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    