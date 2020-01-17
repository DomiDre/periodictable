use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 5,
            name: "Boron",
            symbol: "B",
            mass: 10.811,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(5.30_f64, 0.04_f64)
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
                        mass_number: 7,
                        mass: UncertainFloat::new(7.029_920_f64, 0.000_080_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 8,
                        mass: UncertainFloat::new(8.024_606_7_f64, 0.000_001_2_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 9,
                        mass: UncertainFloat::new(9.013_328_8_f64, 0.000_001_1_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 10,
                        mass: UncertainFloat::new(10.012_937_0_f64, 0.000_000_4_f64),
                        abundance: UncertainFloat::new(19.9_f64, 0.7_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 11,
                        mass: UncertainFloat::new(11.009_305_5_f64, 0.000_000_5_f64),
                        abundance: UncertainFloat::new(80.1_f64, 0.7_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 12,
                        mass: UncertainFloat::new(12.014_352_1_f64, 0.000_001_5_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 13,
                        mass: UncertainFloat::new(13.017_780_3_f64, 0.000_001_2_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 14,
                        mass: UncertainFloat::new(14.025_404_f64, 0.000_023_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 15,
                        mass: UncertainFloat::new(15.031_097_f64, 0.000_024_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 16,
                        mass: UncertainFloat::new(16.039_810_f64, 0.000_060_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 17,
                        mass: UncertainFloat::new(17.046_93_f64, 0.000_15_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 18,
                        mass: UncertainFloat::new(18.056_17_f64, 0.000_86_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 19,
                        mass: UncertainFloat::new(19.063_73_f64, 0.000_43_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    