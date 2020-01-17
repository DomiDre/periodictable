use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 1,
            name: "Hydrogen",
            symbol: "H",
            mass: 1.00794,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-3.740_9_f64, 0.001_1_f64)
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
                        mass_number: 1,
                        mass: UncertainFloat::new(1.007_825_032_1_f64, 0.000_000_000_4_f64),
                        abundance: UncertainFloat::new(99.988_5_f64, 0.007_0_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 2,
                        mass: UncertainFloat::new(2.014_101_778_0_f64, 0.000_000_000_4_f64),
                        abundance: UncertainFloat::new(0.011_5_f64, 0.007_0_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 3,
                        mass: UncertainFloat::new(3.016_049_267_5_f64, 0.000_000_001_1_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 4,
                        mass: UncertainFloat::new(4.027_83_f64, 0.000_12_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 5,
                        mass: UncertainFloat::new(5.039_54_f64, 0.001_02_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 6,
                        mass: UncertainFloat::new(6.044_94_f64, 0.000_28_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    