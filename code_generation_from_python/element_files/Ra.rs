use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 88,
            name: "Radium",
            symbol: "Ra",
            mass: 226.0,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(10.0_f64, 1.0_f64)
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
                        mass_number: 203,
                        mass: UncertainFloat::new(203.009_21_f64, 0.000_25_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 204,
                        mass: UncertainFloat::new(204.006_48_f64, 0.000_19_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 205,
                        mass: UncertainFloat::new(205.006_19_f64, 0.000_22_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 206,
                        mass: UncertainFloat::new(206.003_78_f64, 0.000_16_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 207,
                        mass: UncertainFloat::new(207.003_73_f64, 0.000_45_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 208,
                        mass: UncertainFloat::new(208.001_78_f64, 0.000_15_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 209,
                        mass: UncertainFloat::new(209.001_94_f64, 0.000_13_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 210,
                        mass: UncertainFloat::new(210.000_45_f64, 0.000_10_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 211,
                        mass: UncertainFloat::new(211.000_890_f64, 0.000_070_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 212,
                        mass: UncertainFloat::new(211.999_783_f64, 0.000_015_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 213,
                        mass: UncertainFloat::new(213.000_350_f64, 0.000_030_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 214,
                        mass: UncertainFloat::new(214.000_091_f64, 0.000_012_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 215,
                        mass: UncertainFloat::new(215.002_704_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 216,
                        mass: UncertainFloat::new(216.003_518_f64, 0.000_010_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 217,
                        mass: UncertainFloat::new(217.006_306_f64, 0.000_010_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 218,
                        mass: UncertainFloat::new(218.007_124_f64, 0.000_012_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 219,
                        mass: UncertainFloat::new(219.010_069_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 220,
                        mass: UncertainFloat::new(220.011_015_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 221,
                        mass: UncertainFloat::new(221.013_908_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 222,
                        mass: UncertainFloat::new(222.015_362_f64, 0.000_005_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 223,
                        mass: UncertainFloat::new(223.018_497_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 224,
                        mass: UncertainFloat::new(224.020_202_0_f64, 0.000_002_9_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 225,
                        mass: UncertainFloat::new(225.023_604_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 226,
                        mass: UncertainFloat::new(226.025_402_6_f64, 0.000_002_7_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 227,
                        mass: UncertainFloat::new(227.029_170_7_f64, 0.000_002_7_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 228,
                        mass: UncertainFloat::new(228.031_064_1_f64, 0.000_002_7_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 229,
                        mass: UncertainFloat::new(229.034_820_f64, 0.000_070_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 230,
                        mass: UncertainFloat::new(230.037_080_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 231,
                        mass: UncertainFloat::new(231.041_22_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 232,
                        mass: UncertainFloat::new(232.043_69_f64, 0.000_39_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 233,
                        mass: UncertainFloat::new(233.048_00_f64, 0.000_50_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 234,
                        mass: UncertainFloat::new(234.050_55_f64, 0.000_58_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    