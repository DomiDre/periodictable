use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 14,
            name: "Silicon",
            symbol: "Si",
            mass: 28.0855,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.150_71_f64, 0.000_22_f64)
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
                        mass_number: 22,
                        mass: UncertainFloat::new(22.034_53_f64, 0.000_22_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 23,
                        mass: UncertainFloat::new(23.025_52_f64, 0.000_21_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 24,
                        mass: UncertainFloat::new(24.011_546_f64, 0.000_021_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 25,
                        mass: UncertainFloat::new(25.004_107_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 26,
                        mass: UncertainFloat::new(25.992_330_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 27,
                        mass: UncertainFloat::new(26.986_704_76_f64, 0.000_000_17_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 28,
                        mass: UncertainFloat::new(27.976_926_532_7_f64, 0.000_000_002_0_f64),
                        abundance: UncertainFloat::new(92.229_7_f64, 0.000_7_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 29,
                        mass: UncertainFloat::new(28.976_494_72_f64, 0.000_000_03_f64),
                        abundance: UncertainFloat::new(4.683_2_f64, 0.000_5_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 30,
                        mass: UncertainFloat::new(29.973_770_22_f64, 0.000_000_05_f64),
                        abundance: UncertainFloat::new(3.087_2_f64, 0.000_5_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 31,
                        mass: UncertainFloat::new(30.975_363_27_f64, 0.000_000_07_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 32,
                        mass: UncertainFloat::new(31.974_148_1_f64, 0.000_002_3_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 33,
                        mass: UncertainFloat::new(32.978_001_f64, 0.000_017_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 34,
                        mass: UncertainFloat::new(33.978_576_f64, 0.000_015_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 35,
                        mass: UncertainFloat::new(34.984_580_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 36,
                        mass: UncertainFloat::new(35.986_69_f64, 0.000_11_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 37,
                        mass: UncertainFloat::new(36.993_00_f64, 0.000_13_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 38,
                        mass: UncertainFloat::new(37.995_98_f64, 0.000_29_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 39,
                        mass: UncertainFloat::new(39.002_30_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 40,
                        mass: UncertainFloat::new(40.005_80_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 41,
                        mass: UncertainFloat::new(41.012_70_f64, 0.000_64_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 42,
                        mass: UncertainFloat::new(42.016_10_f64, 0.000_75_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    