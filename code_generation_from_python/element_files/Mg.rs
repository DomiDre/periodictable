use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 12,
            name: "Magnesium",
            symbol: "Mg",
            mass: 24.305,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.375_f64, 0.004_f64)
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
                        mass_number: 20,
                        mass: UncertainFloat::new(20.018_863_f64, 0.000_029_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 21,
                        mass: UncertainFloat::new(21.011_714_f64, 0.000_018_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 22,
                        mass: UncertainFloat::new(21.999_574_1_f64, 0.000_001_5_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 23,
                        mass: UncertainFloat::new(22.994_124_9_f64, 0.000_001_3_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 24,
                        mass: UncertainFloat::new(23.985_041_90_f64, 0.000_000_20_f64),
                        abundance: UncertainFloat::new(78.99_f64, 0.04_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 25,
                        mass: UncertainFloat::new(24.985_837_02_f64, 0.000_000_20_f64),
                        abundance: UncertainFloat::new(10.00_f64, 0.01_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 26,
                        mass: UncertainFloat::new(25.982_593_04_f64, 0.000_000_21_f64),
                        abundance: UncertainFloat::new(11.01_f64, 0.03_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 27,
                        mass: UncertainFloat::new(26.984_340_74_f64, 0.000_000_21_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 28,
                        mass: UncertainFloat::new(27.983_876_7_f64, 0.000_002_2_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 29,
                        mass: UncertainFloat::new(28.988_550_f64, 0.000_030_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 30,
                        mass: UncertainFloat::new(29.990_460_f64, 0.000_070_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 31,
                        mass: UncertainFloat::new(30.996_550_f64, 0.000_080_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 32,
                        mass: UncertainFloat::new(31.999_15_f64, 0.000_10_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 33,
                        mass: UncertainFloat::new(33.005_59_f64, 0.000_16_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 34,
                        mass: UncertainFloat::new(34.009_07_f64, 0.000_28_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 35,
                        mass: UncertainFloat::new(35.017_49_f64, 0.000_47_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 36,
                        mass: UncertainFloat::new(36.022_45_f64, 0.000_97_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 37,
                        mass: UncertainFloat::new(37.031_24_f64, 0.000_97_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    