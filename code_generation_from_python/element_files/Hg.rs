use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 80,
            name: "Mercury",
            symbol: "Hg",
            mass: 200.59,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(12.595_f64, 0.045_f64)
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
                        mass_number: 175,
                        mass: UncertainFloat::new(174.991_41_f64, 0.000_34_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 176,
                        mass: UncertainFloat::new(175.987_410_f64, 0.000_040_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 177,
                        mass: UncertainFloat::new(176.986_34_f64, 0.000_12_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 178,
                        mass: UncertainFloat::new(177.982_476_f64, 0.000_016_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 179,
                        mass: UncertainFloat::new(178.981_78_f64, 0.000_33_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 180,
                        mass: UncertainFloat::new(179.978_32_f64, 0.000_21_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 181,
                        mass: UncertainFloat::new(180.977_81_f64, 0.000_33_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 182,
                        mass: UncertainFloat::new(181.974_75_f64, 0.000_50_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 183,
                        mass: UncertainFloat::new(182.974_56_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 184,
                        mass: UncertainFloat::new(183.971_90_f64, 0.000_22_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 185,
                        mass: UncertainFloat::new(184.971_98_f64, 0.000_30_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 186,
                        mass: UncertainFloat::new(185.969_46_f64, 0.000_22_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 187,
                        mass: UncertainFloat::new(186.969_79_f64, 0.000_26_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 188,
                        mass: UncertainFloat::new(187.967_56_f64, 0.000_19_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 189,
                        mass: UncertainFloat::new(188.968_13_f64, 0.000_30_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 190,
                        mass: UncertainFloat::new(189.966_28_f64, 0.000_16_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 191,
                        mass: UncertainFloat::new(190.967_060_f64, 0.000_090_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 192,
                        mass: UncertainFloat::new(191.965_57_f64, 0.000_30_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 193,
                        mass: UncertainFloat::new(192.966_644_f64, 0.000_021_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 194,
                        mass: UncertainFloat::new(193.965_382_f64, 0.000_025_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 195,
                        mass: UncertainFloat::new(194.966_640_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 196,
                        mass: UncertainFloat::new(195.965_815_f64, 0.000_004_f64),
                        abundance: UncertainFloat::new(0.15_f64, 0.01_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 197,
                        mass: UncertainFloat::new(196.967_195_f64, 0.000_005_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 198,
                        mass: UncertainFloat::new(197.966_752_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(9.97_f64, 0.20_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 199,
                        mass: UncertainFloat::new(198.968_262_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(16.87_f64, 0.22_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 200,
                        mass: UncertainFloat::new(199.968_309_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(23.10_f64, 0.19_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 201,
                        mass: UncertainFloat::new(200.970_285_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(13.18_f64, 0.09_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 202,
                        mass: UncertainFloat::new(201.970_626_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(29.86_f64, 0.26_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 203,
                        mass: UncertainFloat::new(202.972_857_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 204,
                        mass: UncertainFloat::new(203.973_476_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(6.87_f64, 0.15_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 205,
                        mass: UncertainFloat::new(204.976_056_f64, 0.000_005_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 206,
                        mass: UncertainFloat::new(205.977_499_f64, 0.000_022_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 207,
                        mass: UncertainFloat::new(206.982_58_f64, 0.000_16_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 208,
                        mass: UncertainFloat::new(207.985_94_f64, 0.000_32_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    