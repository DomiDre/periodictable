use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 36,
            name: "Krypton",
            symbol: "Kr",
            mass: 83.798,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.81_f64, 0.02_f64)
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
                        mass_number: 69,
                        mass: UncertainFloat::new(68.965_32_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 70,
                        mass: UncertainFloat::new(69.956_01_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 71,
                        mass: UncertainFloat::new(70.950_51_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 72,
                        mass: UncertainFloat::new(71.941_91_f64, 0.000_29_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 73,
                        mass: UncertainFloat::new(72.938_93_f64, 0.000_15_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 74,
                        mass: UncertainFloat::new(73.933_260_f64, 0.000_060_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 75,
                        mass: UncertainFloat::new(74.931_034_f64, 0.000_017_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 76,
                        mass: UncertainFloat::new(75.925_948_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 77,
                        mass: UncertainFloat::new(76.924_668_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 78,
                        mass: UncertainFloat::new(77.920_386_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.35_f64, 0.01_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 79,
                        mass: UncertainFloat::new(78.920_083_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 80,
                        mass: UncertainFloat::new(79.916_378_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(2.28_f64, 0.06_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 81,
                        mass: UncertainFloat::new(80.916_592_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 82,
                        mass: UncertainFloat::new(81.913_484_6_f64, 0.000_002_8_f64),
                        abundance: UncertainFloat::new(11.58_f64, 0.14_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 83,
                        mass: UncertainFloat::new(82.914_136_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(11.49_f64, 0.06_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 84,
                        mass: UncertainFloat::new(83.911_507_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(57.00_f64, 0.04_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 85,
                        mass: UncertainFloat::new(84.912_527_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 86,
                        mass: UncertainFloat::new(85.910_610_3_f64, 0.000_001_2_f64),
                        abundance: UncertainFloat::new(17.30_f64, 0.22_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 87,
                        mass: UncertainFloat::new(86.913_354_3_f64, 0.000_001_4_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 88,
                        mass: UncertainFloat::new(87.914_447_f64, 0.000_014_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 89,
                        mass: UncertainFloat::new(88.917_630_f64, 0.000_060_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 90,
                        mass: UncertainFloat::new(89.919_524_f64, 0.000_020_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 91,
                        mass: UncertainFloat::new(90.923_440_f64, 0.000_060_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 92,
                        mass: UncertainFloat::new(91.926_153_f64, 0.000_013_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 93,
                        mass: UncertainFloat::new(92.931_27_f64, 0.000_11_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 94,
                        mass: UncertainFloat::new(93.934_36_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 95,
                        mass: UncertainFloat::new(94.939_84_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 96,
                        mass: UncertainFloat::new(95.943_07_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 97,
                        mass: UncertainFloat::new(96.948_56_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    