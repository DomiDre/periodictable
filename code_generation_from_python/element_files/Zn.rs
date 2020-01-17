use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 30,
            name: "Zinc",
            symbol: "Zn",
            mass: 65.409,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.680_f64, 0.005_f64)
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
                        mass_number: 54,
                        mass: UncertainFloat::new(53.992_95_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 55,
                        mass: UncertainFloat::new(54.983_98_f64, 0.000_27_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 56,
                        mass: UncertainFloat::new(55.972_38_f64, 0.000_28_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 57,
                        mass: UncertainFloat::new(56.964_91_f64, 0.000_15_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 58,
                        mass: UncertainFloat::new(57.954_600_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 59,
                        mass: UncertainFloat::new(58.949_270_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 60,
                        mass: UncertainFloat::new(59.941_832_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 61,
                        mass: UncertainFloat::new(60.939_514_f64, 0.000_018_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 62,
                        mass: UncertainFloat::new(61.934_334_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 63,
                        mass: UncertainFloat::new(62.933_215_6_f64, 0.000_002_3_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 64,
                        mass: UncertainFloat::new(63.929_146_6_f64, 0.000_001_8_f64),
                        abundance: UncertainFloat::new(48.63_f64, 0.60_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 65,
                        mass: UncertainFloat::new(64.929_245_1_f64, 0.000_001_8_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 66,
                        mass: UncertainFloat::new(65.926_036_8_f64, 0.000_001_6_f64),
                        abundance: UncertainFloat::new(27.90_f64, 0.27_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 67,
                        mass: UncertainFloat::new(66.927_130_9_f64, 0.000_001_7_f64),
                        abundance: UncertainFloat::new(4.10_f64, 0.13_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 68,
                        mass: UncertainFloat::new(67.924_847_6_f64, 0.000_001_7_f64),
                        abundance: UncertainFloat::new(18.75_f64, 0.51_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 69,
                        mass: UncertainFloat::new(68.926_553_5_f64, 0.000_001_8_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 70,
                        mass: UncertainFloat::new(69.925_325_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.62_f64, 0.03_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 71,
                        mass: UncertainFloat::new(70.927_727_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 72,
                        mass: UncertainFloat::new(71.926_861_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 73,
                        mass: UncertainFloat::new(72.929_780_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 74,
                        mass: UncertainFloat::new(73.929_460_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 75,
                        mass: UncertainFloat::new(74.932_940_f64, 0.000_080_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 76,
                        mass: UncertainFloat::new(75.933_39_f64, 0.000_13_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 77,
                        mass: UncertainFloat::new(76.937_09_f64, 0.000_14_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 78,
                        mass: UncertainFloat::new(77.938_57_f64, 0.000_17_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 79,
                        mass: UncertainFloat::new(78.942_68_f64, 0.000_29_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 80,
                        mass: UncertainFloat::new(79.944_41_f64, 0.000_18_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 81,
                        mass: UncertainFloat::new(80.950_48_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 82,
                        mass: UncertainFloat::new(81.954_84_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    