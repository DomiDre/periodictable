use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 29,
            name: "Copper",
            symbol: "Cu",
            mass: 63.546,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.718_f64, 0.004_f64)
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
                        mass_number: 52,
                        mass: UncertainFloat::new(51.997_18_f64, 0.000_28_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 53,
                        mass: UncertainFloat::new(52.985_55_f64, 0.000_28_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 54,
                        mass: UncertainFloat::new(53.976_71_f64, 0.000_23_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 55,
                        mass: UncertainFloat::new(54.966_05_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 56,
                        mass: UncertainFloat::new(55.958_56_f64, 0.000_15_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 57,
                        mass: UncertainFloat::new(56.949_216_f64, 0.000_017_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 58,
                        mass: UncertainFloat::new(57.944_540_7_f64, 0.000_002_7_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 59,
                        mass: UncertainFloat::new(58.939_504_1_f64, 0.000_001_8_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 60,
                        mass: UncertainFloat::new(59.937_368_1_f64, 0.000_002_7_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 61,
                        mass: UncertainFloat::new(60.933_462_2_f64, 0.000_001_9_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 62,
                        mass: UncertainFloat::new(61.932_587_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 63,
                        mass: UncertainFloat::new(62.929_601_1_f64, 0.000_001_5_f64),
                        abundance: UncertainFloat::new(69.17_f64, 0.03_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 64,
                        mass: UncertainFloat::new(63.929_767_9_f64, 0.000_001_5_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 65,
                        mass: UncertainFloat::new(64.927_793_7_f64, 0.000_001_9_f64),
                        abundance: UncertainFloat::new(30.83_f64, 0.03_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 66,
                        mass: UncertainFloat::new(65.928_873_0_f64, 0.000_001_9_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 67,
                        mass: UncertainFloat::new(66.927_750_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 68,
                        mass: UncertainFloat::new(67.929_640_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 69,
                        mass: UncertainFloat::new(68.929_425_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 70,
                        mass: UncertainFloat::new(69.932_409_f64, 0.000_016_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 71,
                        mass: UncertainFloat::new(70.932_620_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 72,
                        mass: UncertainFloat::new(71.935_52_f64, 0.000_21_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 73,
                        mass: UncertainFloat::new(72.936_49_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 74,
                        mass: UncertainFloat::new(73.940_20_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 75,
                        mass: UncertainFloat::new(74.941_70_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 76,
                        mass: UncertainFloat::new(75.945_99_f64, 0.000_64_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 77,
                        mass: UncertainFloat::new(76.947_95_f64, 0.000_75_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 78,
                        mass: UncertainFloat::new(77.952_81_f64, 0.000_86_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 79,
                        mass: UncertainFloat::new(78.955_28_f64, 0.000_97_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 80,
                        mass: UncertainFloat::new(79.961_89_f64, 0.000_97_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    