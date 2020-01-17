use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 8,
            name: "Oxygen",
            symbol: "O",
            mass: 15.9994,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.805_f64, 0.004_f64)
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
                        mass_number: 12,
                        mass: UncertainFloat::new(12.034_405_f64, 0.000_020_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 13,
                        mass: UncertainFloat::new(13.024_810_f64, 0.000_010_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 14,
                        mass: UncertainFloat::new(14.008_595_29_f64, 0.000_000_08_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 15,
                        mass: UncertainFloat::new(15.003_065_4_f64, 0.000_000_5_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 16,
                        mass: UncertainFloat::new(15.994_914_622_1_f64, 0.000_000_001_5_f64),
                        abundance: UncertainFloat::new(99.757_f64, 0.016_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 17,
                        mass: UncertainFloat::new(16.999_131_50_f64, 0.000_000_22_f64),
                        abundance: UncertainFloat::new(0.038_f64, 0.001_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 18,
                        mass: UncertainFloat::new(17.999_160_4_f64, 0.000_000_9_f64),
                        abundance: UncertainFloat::new(0.205_f64, 0.014_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 19,
                        mass: UncertainFloat::new(19.003_579_f64, 0.000_003_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 20,
                        mass: UncertainFloat::new(20.004_076_2_f64, 0.000_001_3_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 21,
                        mass: UncertainFloat::new(21.008_655_f64, 0.000_013_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 22,
                        mass: UncertainFloat::new(22.009_970_f64, 0.000_060_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 23,
                        mass: UncertainFloat::new(23.015_69_f64, 0.000_11_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 24,
                        mass: UncertainFloat::new(24.020_37_f64, 0.000_33_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 25,
                        mass: UncertainFloat::new(25.029_14_f64, 0.000_40_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 26,
                        mass: UncertainFloat::new(26.037_75_f64, 0.000_46_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    