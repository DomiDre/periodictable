use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 16,
            name: "Sulfur",
            symbol: "S",
            mass: 32.065,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(2.847_f64, 0.001_f64)
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
                        mass_number: 26,
                        mass: UncertainFloat::new(26.027_88_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 27,
                        mass: UncertainFloat::new(27.018_80_f64, 0.000_22_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 28,
                        mass: UncertainFloat::new(28.004_37_f64, 0.000_17_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 29,
                        mass: UncertainFloat::new(28.996_610_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 30,
                        mass: UncertainFloat::new(29.984_903_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 31,
                        mass: UncertainFloat::new(30.979_554_4_f64, 0.000_001_6_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 32,
                        mass: UncertainFloat::new(31.972_070_69_f64, 0.000_000_12_f64),
                        abundance: UncertainFloat::new(94.93_f64, 0.31_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 33,
                        mass: UncertainFloat::new(32.971_458_50_f64, 0.000_000_12_f64),
                        abundance: UncertainFloat::new(0.76_f64, 0.02_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 34,
                        mass: UncertainFloat::new(33.967_866_83_f64, 0.000_000_11_f64),
                        abundance: UncertainFloat::new(4.29_f64, 0.28_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 35,
                        mass: UncertainFloat::new(34.969_032_14_f64, 0.000_000_10_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 36,
                        mass: UncertainFloat::new(35.967_080_88_f64, 0.000_000_25_f64),
                        abundance: UncertainFloat::new(0.02_f64, 0.01_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 37,
                        mass: UncertainFloat::new(36.971_125_72_f64, 0.000_000_27_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 38,
                        mass: UncertainFloat::new(37.971_163_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 39,
                        mass: UncertainFloat::new(38.975_140_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 40,
                        mass: UncertainFloat::new(39.975_47_f64, 0.000_25_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 41,
                        mass: UncertainFloat::new(40.980_03_f64, 0.000_23_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 42,
                        mass: UncertainFloat::new(41.981_49_f64, 0.000_35_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 43,
                        mass: UncertainFloat::new(42.986_60_f64, 0.000_90_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 44,
                        mass: UncertainFloat::new(43.988_32_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 45,
                        mass: UncertainFloat::new(44.994_82_f64, 0.000_64_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 46,
                        mass: UncertainFloat::new(45.999_57_f64, 0.000_75_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 47,
                        mass: UncertainFloat::new(47.007_62_f64, 0.000_86_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 48,
                        mass: UncertainFloat::new(48.012_99_f64, 0.000_97_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 49,
                        mass: UncertainFloat::new(49.022_01_f64, 0.001_07_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    