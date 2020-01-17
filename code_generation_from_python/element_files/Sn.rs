use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 50,
            name: "Tin",
            symbol: "Sn",
            mass: 118.71,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(6.225_f64, 0.002_f64)
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
                        mass_number: 100,
                        mass: UncertainFloat::new(99.938_95_f64, 0.000_46_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 101,
                        mass: UncertainFloat::new(100.936_06_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 102,
                        mass: UncertainFloat::new(101.930_49_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 103,
                        mass: UncertainFloat::new(102.928_13_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 104,
                        mass: UncertainFloat::new(103.923_19_f64, 0.000_16_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 105,
                        mass: UncertainFloat::new(104.921_39_f64, 0.000_10_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 106,
                        mass: UncertainFloat::new(105.916_880_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 107,
                        mass: UncertainFloat::new(106.915_670_f64, 0.000_090_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 108,
                        mass: UncertainFloat::new(107.911_970_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 109,
                        mass: UncertainFloat::new(108.911_287_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 110,
                        mass: UncertainFloat::new(109.907_853_f64, 0.000_017_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 111,
                        mass: UncertainFloat::new(110.907_735_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 112,
                        mass: UncertainFloat::new(111.904_821_f64, 0.000_005_f64),
                        abundance: UncertainFloat::new(0.97_f64, 0.01_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 113,
                        mass: UncertainFloat::new(112.905_173_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 114,
                        mass: UncertainFloat::new(113.902_782_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.66_f64, 0.01_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 115,
                        mass: UncertainFloat::new(114.903_346_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.34_f64, 0.01_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 116,
                        mass: UncertainFloat::new(115.901_744_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(14.54_f64, 0.09_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 117,
                        mass: UncertainFloat::new(116.902_954_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(7.68_f64, 0.07_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 118,
                        mass: UncertainFloat::new(117.901_606_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(24.22_f64, 0.09_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 119,
                        mass: UncertainFloat::new(118.903_309_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(8.59_f64, 0.04_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 120,
                        mass: UncertainFloat::new(119.902_196_6_f64, 0.000_002_7_f64),
                        abundance: UncertainFloat::new(32.58_f64, 0.09_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 121,
                        mass: UncertainFloat::new(120.904_236_9_f64, 0.000_002_7_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 122,
                        mass: UncertainFloat::new(121.903_440_1_f64, 0.000_002_9_f64),
                        abundance: UncertainFloat::new(4.63_f64, 0.03_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 123,
                        mass: UncertainFloat::new(122.905_721_9_f64, 0.000_002_9_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 124,
                        mass: UncertainFloat::new(123.905_274_6_f64, 0.000_001_5_f64),
                        abundance: UncertainFloat::new(5.79_f64, 0.05_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 125,
                        mass: UncertainFloat::new(124.907_784_9_f64, 0.000_001_6_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 126,
                        mass: UncertainFloat::new(125.907_654_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 127,
                        mass: UncertainFloat::new(126.910_351_f64, 0.000_027_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 128,
                        mass: UncertainFloat::new(127.910_535_f64, 0.000_029_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 129,
                        mass: UncertainFloat::new(128.913_44_f64, 0.000_13_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 130,
                        mass: UncertainFloat::new(129.913_850_f64, 0.000_030_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 131,
                        mass: UncertainFloat::new(130.916_920_f64, 0.000_080_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 132,
                        mass: UncertainFloat::new(131.917_744_f64, 0.000_028_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 133,
                        mass: UncertainFloat::new(132.923_810_f64, 0.000_090_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 134,
                        mass: UncertainFloat::new(133.928_46_f64, 0.000_11_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 135,
                        mass: UncertainFloat::new(134.934_73_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 136,
                        mass: UncertainFloat::new(135.939_34_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 137,
                        mass: UncertainFloat::new(136.945_79_f64, 0.000_64_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    