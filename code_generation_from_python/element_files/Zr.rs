use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 40,
            name: "Zirconium",
            symbol: "Zr",
            mass: 91.224,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.16_f64, 0.03_f64)
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
                        mass_number: 79,
                        mass: UncertainFloat::new(78.949_16_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 80,
                        mass: UncertainFloat::new(79.940_55_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 81,
                        mass: UncertainFloat::new(80.936_82_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 82,
                        mass: UncertainFloat::new(81.931_09_f64, 0.000_55_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 83,
                        mass: UncertainFloat::new(82.928_65_f64, 0.000_10_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 84,
                        mass: UncertainFloat::new(83.923_25_f64, 0.000_21_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 85,
                        mass: UncertainFloat::new(84.921_47_f64, 0.000_11_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 86,
                        mass: UncertainFloat::new(85.916_470_f64, 0.000_030_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 87,
                        mass: UncertainFloat::new(86.914_817_f64, 0.000_009_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 88,
                        mass: UncertainFloat::new(87.910_226_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 89,
                        mass: UncertainFloat::new(88.908_889_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 90,
                        mass: UncertainFloat::new(89.904_703_7_f64, 0.000_002_3_f64),
                        abundance: UncertainFloat::new(51.45_f64, 0.40_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 91,
                        mass: UncertainFloat::new(90.905_645_0_f64, 0.000_002_3_f64),
                        abundance: UncertainFloat::new(11.22_f64, 0.05_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 92,
                        mass: UncertainFloat::new(91.905_040_1_f64, 0.000_002_3_f64),
                        abundance: UncertainFloat::new(17.15_f64, 0.08_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 93,
                        mass: UncertainFloat::new(92.906_475_6_f64, 0.000_002_3_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 94,
                        mass: UncertainFloat::new(93.906_315_8_f64, 0.000_002_5_f64),
                        abundance: UncertainFloat::new(17.38_f64, 0.28_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 95,
                        mass: UncertainFloat::new(94.908_042_7_f64, 0.000_002_5_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 96,
                        mass: UncertainFloat::new(95.908_276_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(2.80_f64, 0.09_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 97,
                        mass: UncertainFloat::new(96.910_951_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 98,
                        mass: UncertainFloat::new(97.912_746_f64, 0.000_021_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 99,
                        mass: UncertainFloat::new(98.916_511_f64, 0.000_021_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 100,
                        mass: UncertainFloat::new(99.917_760_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 101,
                        mass: UncertainFloat::new(100.921_140_f64, 0.000_030_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 102,
                        mass: UncertainFloat::new(101.922_980_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 103,
                        mass: UncertainFloat::new(102.926_60_f64, 0.000_12_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 104,
                        mass: UncertainFloat::new(103.928_78_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 105,
                        mass: UncertainFloat::new(104.933_05_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 106,
                        mass: UncertainFloat::new(105.935_91_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 107,
                        mass: UncertainFloat::new(106.940_86_f64, 0.000_64_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 108,
                        mass: UncertainFloat::new(107.944_28_f64, 0.000_75_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    