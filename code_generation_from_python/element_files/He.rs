use crate::Element;
    use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

    pub fn load() -> Element {
        Element {
            atomic_number: 2,
            name: "Helium",
            symbol: "He",
            mass: 4.002602,
            common_ions: vec![-1, 1],
            oxidation_states: vec![],
            neutron_b_coherent: NeutronScatteringFactor {
                    b_c: UncertainFloat::new(3.26_f64, 0.03_f64)
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
                        mass_number: 3,
                        mass: UncertainFloat::new(3.016_029_309_7_f64, 0.000_000_000_9_f64),
                        abundance: UncertainFloat::new(0.000_137_f64, 0.000_003_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 4,
                        mass: UncertainFloat::new(4.002_603_249_7_f64, 0.000_000_001_0_f64),
                        abundance: UncertainFloat::new(99.999_863_f64, 0.000_003_f64),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 5,
                        mass: UncertainFloat::new(5.012_220_f64, 0.000_050_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 6,
                        mass: UncertainFloat::new(6.018_888_1_f64, 0.000_001_1_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 7,
                        mass: UncertainFloat::new(7.028_030_f64, 0.000_030_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 8,
                        mass: UncertainFloat::new(8.033_922_f64, 0.000_008_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 9,
                        mass: UncertainFloat::new(9.043_820_f64, 0.000_070_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                    Isotope { 
                        mass_number: 10,
                        mass: UncertainFloat::new(10.052_400_f64, 0.000_080_f64),
                        abundance: UncertainFloat::new(0.0, 0.0),
                        b_coherent: 
                    },
                ]
        }
    }
    