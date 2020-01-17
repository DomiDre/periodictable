use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 82,
            name: "Lead",
            symbol: "Pb",
            mass: 207.2,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(9.401_f64, 0.002_f64)
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
                        mass_number: 181,
                        mass: UncertainFloat::new(180.996_71_f64, 0.000_17_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 182,
                        mass: UncertainFloat::new(181.992_676_f64, 0.000_018_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 183,
                        mass: UncertainFloat::new(182.991_93_f64, 0.000_33_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 184,
                        mass: UncertainFloat::new(183.988_20_f64, 0.000_21_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 185,
                        mass: UncertainFloat::new(184.987_58_f64, 0.000_33_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 186,
                        mass: UncertainFloat::new(185.984_30_f64, 0.000_50_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 187,
                        mass: UncertainFloat::new(186.984_03_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 188,
                        mass: UncertainFloat::new(187.981_06_f64, 0.000_22_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 189,
                        mass: UncertainFloat::new(188.980_88_f64, 0.000_29_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 190,
                        mass: UncertainFloat::new(189.978_18_f64, 0.000_22_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 191,
                        mass: UncertainFloat::new(190.978_20_f64, 0.000_23_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 192,
                        mass: UncertainFloat::new(191.975_76_f64, 0.000_19_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 193,
                        mass: UncertainFloat::new(192.976_08_f64, 0.000_20_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 194,
                        mass: UncertainFloat::new(193.973_97_f64, 0.000_16_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 195,
                        mass: UncertainFloat::new(194.974_47_f64, 0.000_44_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 196,
                        mass: UncertainFloat::new(195.972_71_f64, 0.000_15_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 197,
                        mass: UncertainFloat::new(196.973_38_f64, 0.000_11_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 198,
                        mass: UncertainFloat::new(197.971_98_f64, 0.000_10_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 199,
                        mass: UncertainFloat::new(198.972_910_f64, 0.000_070_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 200,
                        mass: UncertainFloat::new(199.971_816_f64, 0.000_014_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 201,
                        mass: UncertainFloat::new(200.972_850_f64, 0.000_030_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 202,
                        mass: UncertainFloat::new(201.972_144_f64, 0.000_011_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 203,
                        mass: UncertainFloat::new(202.973_375_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 204,
                        mass: UncertainFloat::new(203.973_029_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(1.4_f64, 0.1_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 205,
                        mass: UncertainFloat::new(204.974_467_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 206,
                        mass: UncertainFloat::new(205.974_449_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(24.1_f64, 0.1_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 207,
                        mass: UncertainFloat::new(206.975_881_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(22.1_f64, 0.1_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 208,
                        mass: UncertainFloat::new(207.976_636_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(52.4_f64, 0.1_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 209,
                        mass: UncertainFloat::new(208.981_075_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 210,
                        mass: UncertainFloat::new(209.984_173_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 211,
                        mass: UncertainFloat::new(210.988_731_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 212,
                        mass: UncertainFloat::new(211.991_887_5_f64, 0.000_002_9_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 213,
                        mass: UncertainFloat::new(212.996_50_f64, 0.000_11_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 214,
                        mass: UncertainFloat::new(213.999_798_1_f64, 0.000_002_7_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    