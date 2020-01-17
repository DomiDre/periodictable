use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 4,
            name: "Beryllium",
            symbol: "Be",
            mass: 9.012182,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.79_f64, 0.01_f64)
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
                        mass_number: 5,
                        mass: UncertainFloat::new(5.040_79_f64, 0.004_29_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 6,
                        mass: UncertainFloat::new(6.019_726_f64, 0.000_006_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 7,
                        mass: UncertainFloat::new(7.016_929_2_f64, 0.000_000_5_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 8,
                        mass: UncertainFloat::new(8.005_305_09_f64, 0.000_000_04_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 9,
                        mass: UncertainFloat::new(9.012_182_1_f64, 0.000_000_4_f64),
                        abundance: 100, 0.0,
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 10,
                        mass: UncertainFloat::new(10.013_533_7_f64, 0.000_000_4_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 11,
                        mass: UncertainFloat::new(11.021_658_f64, 0.000_007_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 12,
                        mass: UncertainFloat::new(12.026_921_f64, 0.000_016_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 13,
                        mass: UncertainFloat::new(13.036_13_f64, 0.000_54_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 14,
                        mass: UncertainFloat::new(14.042_82_f64, 0.000_12_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    