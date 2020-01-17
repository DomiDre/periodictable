use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 37,
            name: "Rubidium",
            symbol: "Rb",
            mass: 85.4678,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.08_f64, 0.02_f64)
                    b_p:
                    b_m:
                    coherent_scattering_xs:
                    incoherent_scattering_xs:
                    absorption_scattering_xs:
                    thermal_absorption_xs:
                }
            isotopes:
                vec![
                    Isotope { 
                        mass_number: 71,
                        mass: UncertainFloat::new(70.965_32_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 72,
                        mass: UncertainFloat::new(71.959_08_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 73,
                        mass: UncertainFloat::new(72.950_37_f64, 0.000_52_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 74,
                        mass: UncertainFloat::new(73.944_47_f64, 0.000_77_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 75,
                        mass: UncertainFloat::new(74.938_569_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 76,
                        mass: UncertainFloat::new(75.935_071_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 77,
                        mass: UncertainFloat::new(76.930_407_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 78,
                        mass: UncertainFloat::new(77.928_141_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 79,
                        mass: UncertainFloat::new(78.923_997_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 80,
                        mass: UncertainFloat::new(79.922_519_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 81,
                        mass: UncertainFloat::new(80.918_994_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 82,
                        mass: UncertainFloat::new(81.918_208_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 83,
                        mass: UncertainFloat::new(82.915_112_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 84,
                        mass: UncertainFloat::new(83.914_385_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 85,
                        mass: UncertainFloat::new(84.911_789_3_f64, 0.000_002_5_f64),
                        abundance: UncertainFloat::new(72.17_f64, 0.02_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 86,
                        mass: UncertainFloat::new(85.911_167_1_f64, 0.000_002_5_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 87,
                        mass: UncertainFloat::new(86.909_183_5_f64, 0.000_002_7_f64),
                        abundance: UncertainFloat::new(27.83_f64, 0.02_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 88,
                        mass: UncertainFloat::new(87.911_319_f64, 0.000_005_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 89,
                        mass: UncertainFloat::new(88.912_280_f64, 0.000_006_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 90,
                        mass: UncertainFloat::new(89.914_809_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 91,
                        mass: UncertainFloat::new(90.916_534_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 92,
                        mass: UncertainFloat::new(91.919_725_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 93,
                        mass: UncertainFloat::new(92.922_033_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 94,
                        mass: UncertainFloat::new(93.926_407_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 95,
                        mass: UncertainFloat::new(94.929_319_f64, 0.000_021_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 96,
                        mass: UncertainFloat::new(95.934_284_f64, 0.000_027_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 97,
                        mass: UncertainFloat::new(96.937_340_f64, 0.000_030_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 98,
                        mass: UncertainFloat::new(97.941_700_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 99,
                        mass: UncertainFloat::new(98.945_42_f64, 0.000_16_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 100,
                        mass: UncertainFloat::new(99.949_87_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 101,
                        mass: UncertainFloat::new(100.953_20_f64, 0.000_18_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 102,
                        mass: UncertainFloat::new(101.959_21_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    