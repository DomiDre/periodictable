use crate::Element;
use crate::{Isotope, UncertainFloat, NeutronScatteringFactor};

pub fn load() -> Element {
    Element {
        atomic_number: 23,
        name: "Vanadium",
        symbol: "V",
        mass: 50.9415,
        common_ions: vec![-1, 1],
        oxidation_states: vec![],
        xray_scattering: None,
        neutron_scattering: Some(NeutronScatteringFactor {
            b_c: UncertainFloat::new(-0.443_f64, 0.014_f64),
            b_p: None,
            b_m: None,
            coherent_scattering_xs: Some(UncertainFloat::new(0.018_38_f64, 0.000_12_f64)),
            incoherent_scattering_xs: Some(UncertainFloat::new(5.08_f64, 0.06_f64)),
            absorption_scattering_xs: Some(UncertainFloat::new(5.10_f64, 0.06_f64)),
            thermal_absorption_xs: Some(UncertainFloat::new(5.08_f64, 0.04_f64)),
        }),
        isotopes: vec![
            Isotope { 
                mass_number: 40,
                mass: UncertainFloat::new(40.011_09_f64, 0.000_54_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 41,
                mass: UncertainFloat::new(40.999_74_f64, 0.000_27_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 42,
                mass: UncertainFloat::new(41.991_23_f64, 0.000_21_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 43,
                mass: UncertainFloat::new(42.980_65_f64, 0.000_25_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 44,
                mass: UncertainFloat::new(43.974_400_f64, 0.000_090_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 45,
                mass: UncertainFloat::new(44.965_782_f64, 0.000_018_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 46,
                mass: UncertainFloat::new(45.960_199_5_f64, 0.000_001_6_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 47,
                mass: UncertainFloat::new(46.954_906_9_f64, 0.000_001_2_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 48,
                mass: UncertainFloat::new(47.952_254_5_f64, 0.000_002_8_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 49,
                mass: UncertainFloat::new(48.948_516_9_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 50,
                mass: UncertainFloat::new(49.947_162_8_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(0.250_f64, 0.004_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(7.6_f64, 0.6_f64),
                    b_p: None,
                    b_m: None,
                    coherent_scattering_xs: Some(UncertainFloat::new(7.3_f64, 1.1_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(0.5_f64, 0.5_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(7.8_f64, 1.0_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(60.0_f64, 40.0_f64)),
                }),
            },
            Isotope { 
                mass_number: 51,
                mass: UncertainFloat::new(50.943_963_7_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(99.750_f64, 0.004_f64),
                xray_scattering: None,
                neutron_scattering: Some(NeutronScatteringFactor {
                    b_c: UncertainFloat::new(-0.402_f64, 0.002_f64),
                    b_p: Some(UncertainFloat::new(4.93_f64, 0.25_f64)),
                    b_m: Some(UncertainFloat::new(-7.58_f64, 0.28_f64)),
                    coherent_scattering_xs: Some(UncertainFloat::new(0.020_3_f64, 0.000_2_f64)),
                    incoherent_scattering_xs: Some(UncertainFloat::new(5.07_f64, 0.06_f64)),
                    absorption_scattering_xs: Some(UncertainFloat::new(5.09_f64, 0.06_f64)),
                    thermal_absorption_xs: Some(UncertainFloat::new(4.9_f64, 0.1_f64)),
                }),
            },
            Isotope { 
                mass_number: 52,
                mass: UncertainFloat::new(51.944_779_7_f64, 0.000_001_4_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 53,
                mass: UncertainFloat::new(52.944_343_f64, 0.000_004_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 54,
                mass: UncertainFloat::new(53.946_444_f64, 0.000_016_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 55,
                mass: UncertainFloat::new(54.947_24_f64, 0.000_11_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 56,
                mass: UncertainFloat::new(55.950_36_f64, 0.000_26_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 57,
                mass: UncertainFloat::new(56.952_36_f64, 0.000_27_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 58,
                mass: UncertainFloat::new(57.956_65_f64, 0.000_28_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 59,
                mass: UncertainFloat::new(58.959_30_f64, 0.000_35_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 60,
                mass: UncertainFloat::new(59.964_50_f64, 0.000_60_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 61,
                mass: UncertainFloat::new(60.967_41_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 62,
                mass: UncertainFloat::new(61.973_14_f64, 0.000_75_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
            Isotope { 
                mass_number: 63,
                mass: UncertainFloat::new(62.976_75_f64, 0.000_97_f64),
                abundance: UncertainFloat::new(0.0, 0.0),
                xray_scattering: None,
                neutron_scattering: None
            },
        ]
    }
}
