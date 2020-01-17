use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 42,
            name: "Molybdenum",
            symbol: "Mo",
            mass: 95.94,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.715_f64, 0.020_f64)
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
                        mass_number: 83,
                        mass: UncertainFloat::new(82.948_74_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 84,
                        mass: UncertainFloat::new(83.940_09_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 85,
                        mass: UncertainFloat::new(84.936_59_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 86,
                        mass: UncertainFloat::new(85.930_70_f64, 0.000_47_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 87,
                        mass: UncertainFloat::new(86.927_33_f64, 0.000_24_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 88,
                        mass: UncertainFloat::new(87.921_953_f64, 0.000_022_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 89,
                        mass: UncertainFloat::new(88.919_481_f64, 0.000_017_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 90,
                        mass: UncertainFloat::new(89.913_936_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 91,
                        mass: UncertainFloat::new(90.911_751_f64, 0.000_012_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 92,
                        mass: UncertainFloat::new(91.906_810_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(14.84_f64, 0.35_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 93,
                        mass: UncertainFloat::new(92.906_812_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 94,
                        mass: UncertainFloat::new(93.905_087_6_f64, 0.000_002_0_f64),
                        abundance: UncertainFloat::new(9.25_f64, 0.12_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 95,
                        mass: UncertainFloat::new(94.905_841_5_f64, 0.000_002_0_f64),
                        abundance: UncertainFloat::new(15.92_f64, 0.13_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 96,
                        mass: UncertainFloat::new(95.904_678_9_f64, 0.000_002_0_f64),
                        abundance: UncertainFloat::new(16.68_f64, 0.02_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 97,
                        mass: UncertainFloat::new(96.906_021_0_f64, 0.000_002_0_f64),
                        abundance: UncertainFloat::new(9.55_f64, 0.08_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 98,
                        mass: UncertainFloat::new(97.905_407_8_f64, 0.000_002_0_f64),
                        abundance: UncertainFloat::new(24.13_f64, 0.31_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 99,
                        mass: UncertainFloat::new(98.907_711_6_f64, 0.000_002_0_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 100,
                        mass: UncertainFloat::new(99.907_477_f64, 0.000_006_f64),
                        abundance: UncertainFloat::new(9.63_f64, 0.23_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 101,
                        mass: UncertainFloat::new(100.910_347_f64, 0.000_006_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 102,
                        mass: UncertainFloat::new(101.910_297_f64, 0.000_022_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 103,
                        mass: UncertainFloat::new(102.913_200_f64, 0.000_070_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 104,
                        mass: UncertainFloat::new(103.913_760_f64, 0.000_070_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 105,
                        mass: UncertainFloat::new(104.916_970_f64, 0.000_080_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 106,
                        mass: UncertainFloat::new(105.918_134_f64, 0.000_023_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 107,
                        mass: UncertainFloat::new(106.921_69_f64, 0.000_17_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 108,
                        mass: UncertainFloat::new(107.923_58_f64, 0.000_21_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 109,
                        mass: UncertainFloat::new(108.927_81_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 110,
                        mass: UncertainFloat::new(109.929_73_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 111,
                        mass: UncertainFloat::new(110.934_51_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 112,
                        mass: UncertainFloat::new(111.936_84_f64, 0.000_64_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 113,
                        mass: UncertainFloat::new(112.942_03_f64, 0.000_64_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    