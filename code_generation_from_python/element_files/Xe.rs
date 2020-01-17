use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 54,
            name: "Xenon",
            symbol: "Xe",
            mass: 131.293,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(4.69_f64, 0.04_f64)
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
                        mass_number: 110,
                        mass: UncertainFloat::new(109.944_48_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 111,
                        mass: UncertainFloat::new(110.941_63_f64, 0.000_33_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 112,
                        mass: UncertainFloat::new(111.935_67_f64, 0.000_16_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 113,
                        mass: UncertainFloat::new(112.933_38_f64, 0.000_10_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 114,
                        mass: UncertainFloat::new(113.928_15_f64, 0.000_22_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 115,
                        mass: UncertainFloat::new(114.926_54_f64, 0.000_26_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 116,
                        mass: UncertainFloat::new(115.921_74_f64, 0.000_26_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 117,
                        mass: UncertainFloat::new(116.920_56_f64, 0.000_19_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 118,
                        mass: UncertainFloat::new(117.916_57_f64, 0.001_07_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 119,
                        mass: UncertainFloat::new(118.915_55_f64, 0.000_13_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 120,
                        mass: UncertainFloat::new(119.912_150_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 121,
                        mass: UncertainFloat::new(120.911_386_f64, 0.000_026_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 122,
                        mass: UncertainFloat::new(121.908_550_f64, 0.000_090_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 123,
                        mass: UncertainFloat::new(122.908_471_f64, 0.000_017_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 124,
                        mass: UncertainFloat::new(123.905_895_8_f64, 0.000_002_1_f64),
                        abundance: UncertainFloat::new(0.09_f64, 0.01_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 125,
                        mass: UncertainFloat::new(124.906_398_2_f64, 0.000_002_1_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 126,
                        mass: UncertainFloat::new(125.904_269_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.09_f64, 0.01_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 127,
                        mass: UncertainFloat::new(126.905_180_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 128,
                        mass: UncertainFloat::new(127.903_530_4_f64, 0.000_001_5_f64),
                        abundance: UncertainFloat::new(1.92_f64, 0.03_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 129,
                        mass: UncertainFloat::new(128.904_779_5_f64, 0.000_000_9_f64),
                        abundance: UncertainFloat::new(26.44_f64, 0.24_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 130,
                        mass: UncertainFloat::new(129.903_507_9_f64, 0.000_001_0_f64),
                        abundance: UncertainFloat::new(4.08_f64, 0.02_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 131,
                        mass: UncertainFloat::new(130.905_081_9_f64, 0.000_001_0_f64),
                        abundance: UncertainFloat::new(21.18_f64, 0.03_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 132,
                        mass: UncertainFloat::new(131.904_154_5_f64, 0.000_001_2_f64),
                        abundance: UncertainFloat::new(26.89_f64, 0.06_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 133,
                        mass: UncertainFloat::new(132.905_906_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 134,
                        mass: UncertainFloat::new(133.905_394_5_f64, 0.000_000_9_f64),
                        abundance: UncertainFloat::new(10.44_f64, 0.10_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 135,
                        mass: UncertainFloat::new(134.907_207_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 136,
                        mass: UncertainFloat::new(135.907_220_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(8.87_f64, 0.16_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 137,
                        mass: UncertainFloat::new(136.911_563_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 138,
                        mass: UncertainFloat::new(137.913_990_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 139,
                        mass: UncertainFloat::new(138.918_787_f64, 0.000_023_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 140,
                        mass: UncertainFloat::new(139.921_640_f64, 0.000_070_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 141,
                        mass: UncertainFloat::new(140.926_65_f64, 0.000_10_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 142,
                        mass: UncertainFloat::new(141.929_70_f64, 0.000_11_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 143,
                        mass: UncertainFloat::new(142.934_89_f64, 0.000_24_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 144,
                        mass: UncertainFloat::new(143.938_23_f64, 0.000_34_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 145,
                        mass: UncertainFloat::new(144.943_67_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 146,
                        mass: UncertainFloat::new(145.947_30_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 147,
                        mass: UncertainFloat::new(146.953_01_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    