use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 35,
            name: "Bromine",
            symbol: "Br",
            mass: 79.904,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.79_f64, 0.02_f64)
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
                        mass_number: 67,
                        mass: UncertainFloat::new(66.964_79_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 68,
                        mass: UncertainFloat::new(67.958_25_f64, 0.000_58_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 69,
                        mass: UncertainFloat::new(68.950_18_f64, 0.000_34_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 70,
                        mass: UncertainFloat::new(69.944_62_f64, 0.000_39_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 71,
                        mass: UncertainFloat::new(70.939_25_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 72,
                        mass: UncertainFloat::new(71.936_50_f64, 0.000_28_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 73,
                        mass: UncertainFloat::new(72.931_79_f64, 0.000_14_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 74,
                        mass: UncertainFloat::new(73.929_891_f64, 0.000_016_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 75,
                        mass: UncertainFloat::new(74.925_776_f64, 0.000_015_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 76,
                        mass: UncertainFloat::new(75.924_542_f64, 0.000_010_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 77,
                        mass: UncertainFloat::new(76.921_380_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 78,
                        mass: UncertainFloat::new(77.921_146_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 79,
                        mass: UncertainFloat::new(78.918_337_6_f64, 0.000_002_0_f64),
                        abundance: UncertainFloat::new(50.69_f64, 0.07_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 80,
                        mass: UncertainFloat::new(79.918_530_0_f64, 0.000_002_0_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 81,
                        mass: UncertainFloat::new(80.916_291_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(49.31_f64, 0.07_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 82,
                        mass: UncertainFloat::new(81.916_805_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 83,
                        mass: UncertainFloat::new(82.915_180_f64, 0.000_005_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 84,
                        mass: UncertainFloat::new(83.916_504_f64, 0.000_027_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 85,
                        mass: UncertainFloat::new(84.915_608_f64, 0.000_021_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 86,
                        mass: UncertainFloat::new(85.918_797_f64, 0.000_012_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 87,
                        mass: UncertainFloat::new(86.920_711_f64, 0.000_019_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 88,
                        mass: UncertainFloat::new(87.924_070_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 89,
                        mass: UncertainFloat::new(88.926_390_f64, 0.000_060_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 90,
                        mass: UncertainFloat::new(89.930_630_f64, 0.000_080_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 91,
                        mass: UncertainFloat::new(90.933_970_f64, 0.000_080_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 92,
                        mass: UncertainFloat::new(91.939_260_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 93,
                        mass: UncertainFloat::new(92.943_10_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 94,
                        mass: UncertainFloat::new(93.948_68_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    